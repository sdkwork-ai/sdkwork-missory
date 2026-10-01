//! Derived reminder engine (PRD §27): long-uncontacted, birthday, commitment,
//! important events, and story anniversaries.

use std::sync::Arc;

use sdkwork_missory_contract::dto::{
    Importance, MemoryStatus, MemoryType, Person, Reminder, ReminderType,
};
use sdkwork_missory_contract::error::{MissoryServiceError, MissoryServiceResult};
use sdkwork_missory_contract::ports::{ListPersonsQuery, ListStoriesQuery};
use sdkwork_missory_spi::{MissoryScope, MissoryStore, ReminderStateRecord, ReminderStateStatus};
use time::OffsetDateTime;

use crate::service::{format_rfc3339, parse_rfc3339};

/// Birthday lead time in days.
pub const BIRTHDAY_LEAD_DAYS: i64 = 30;
/// Anniversary lead time in days.
pub const ANNIVERSARY_LEAD_DAYS: i64 = 7;
/// How far back an important event still counts as recent.
pub const IMPORTANT_EVENT_WINDOW_DAYS: i64 = 30;

/// Reminder key prefixes accepted by the dismiss/snooze commands.
const REMINDER_KEY_PREFIXES: [&str; 5] = [
    "birthday:",
    "uncontacted:",
    "commitment:",
    "event:",
    "anniversary:",
];

/// Validates `MM-DD` or `YYYY-MM-DD` birthday formats.
pub fn is_valid_birthday(birthday: &str) -> bool {
    parse_birthday(birthday).is_some()
}

/// Parses a birthday into `(month, day)`; accepts `MM-DD` and `YYYY-MM-DD`.
pub fn parse_birthday(birthday: &str) -> Option<(u8, u8)> {
    let parts: Vec<&str> = birthday.split('-').collect();
    let (month_raw, day_raw) = match parts.as_slice() {
        [month, day] => (*month, *day),
        [year, month, day] => {
            if year.len() != 4 || !year.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            (*month, *day)
        }
        _ => return None,
    };
    if month_raw.len() != 2 || day_raw.len() != 2 {
        return None;
    }
    let month: u8 = month_raw.parse().ok()?;
    let day: u8 = day_raw.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((month, day))
}

/// Computes the next occurrence of a birthday relative to `today`.
fn next_birthday(birthday: &str, today: OffsetDateTime) -> Option<(OffsetDateTime, i64)> {
    let (month, day) = parse_birthday(birthday)?;
    let month_enum = time::Month::try_from(month).ok()?;
    let this_year = today.year();
    let this_occurrence = time::Date::from_calendar_date(this_year, month_enum, day).ok()?;
    let next = if this_occurrence >= today.date() {
        this_occurrence
    } else {
        time::Date::from_calendar_date(this_year + 1, month_enum, day).ok()?
    };
    let days_until = (next - today.date()).whole_days();
    let instant = next.with_time(time::Time::MIDNIGHT).assume_utc();
    Some((instant, days_until))
}

/// Derives every currently-active reminder for the owner scope.
pub async fn derive_reminders(
    store: &Arc<dyn MissoryStore>,
    scope: &MissoryScope,
    now: OffsetDateTime,
    default_cycle_days: i64,
) -> sdkwork_missory_spi::MissoryStoreResult<Vec<Reminder>> {
    let mut reminders = Vec::new();
    let (persons, _) = store
        .list_persons(
            scope,
            ListPersonsQuery {
                page: Some(1),
                page_size: Some(200),
                ..ListPersonsQuery::default()
            },
        )
        .await?;
    for person in persons {
        append_person_reminders(
            store,
            scope,
            &person,
            now,
            default_cycle_days,
            &mut reminders,
        )
        .await?;
    }
    append_story_anniversaries(store, scope, now, &mut reminders).await?;
    reminders.sort_by(|left, right| left.due_at.cmp(&right.due_at));
    Ok(reminders)
}

async fn append_person_reminders(
    store: &Arc<dyn MissoryStore>,
    scope: &MissoryScope,
    person: &Person,
    now: OffsetDateTime,
    default_cycle_days: i64,
    out: &mut Vec<Reminder>,
) -> sdkwork_missory_spi::MissoryStoreResult<()> {
    let relationship = store.get_relationship(scope, person.id).await?;

    // Long-uncontacted, anchored on the last contact (or meeting/creation).
    let anchor = relationship
        .as_ref()
        .and_then(|edge| edge.last_contacted_at.as_deref())
        .or(person.last_contacted_at.as_deref())
        .and_then(parse_rfc3339)
        .or_else(|| {
            relationship
                .as_ref()
                .and_then(|edge| edge.started_at.as_deref())
                .and_then(parse_rfc3339)
        })
        .or_else(|| parse_rfc3339(&person.created_at));
    if let Some(anchor) = anchor {
        let cycle = relationship
            .as_ref()
            .and_then(|edge| edge.contact_cycle_days)
            .unwrap_or(default_cycle_days)
            .max(1);
        let days_silent = (now - anchor).whole_days();
        let overdue = days_silent - cycle;
        if overdue >= 0 {
            out.push(Reminder {
                reminder_id: format!("uncontacted:{}", person.id),
                reminder_type: ReminderType::LongUncontacted,
                person_id: person.id,
                person_name: person.display_name.clone(),
                title: format!("已经 {days_silent} 天没有联系"),
                detail: Some(format!("超过了你设定的 {cycle} 天联系周期")),
                due_at: format_rfc3339(now),
                days_overdue: Some(overdue),
                days_until: None,
            });
        }
    }

    // Birthday within the lead window.
    if let Some(birthday) = person.birthday.as_deref() {
        if let Some((instant, days_until)) =
            next_birthday(birthday, now).filter(|(_, days)| *days <= BIRTHDAY_LEAD_DAYS)
        {
            out.push(Reminder {
                reminder_id: format!("birthday:{}", person.id),
                reminder_type: ReminderType::Birthday,
                person_id: person.id,
                person_name: person.display_name.clone(),
                title: format!("生日还有 {days_until} 天"),
                detail: Some(format!("生日 {birthday}")),
                due_at: format_rfc3339(instant),
                days_overdue: None,
                days_until: Some(days_until),
            });
        }
    }

    // Open commitments and important recent events.
    let memories = store
        .memories_for_person(scope, person.id, usize::MAX >> 4)
        .await?;
    append_memory_reminders(person, &memories, now, out);
    Ok(())
}

/// Pushes commitment and important-event reminders derived from one person's memories.
fn append_memory_reminders(
    person: &Person,
    memories: &[sdkwork_missory_contract::dto::Memory],
    now: OffsetDateTime,
    out: &mut Vec<Reminder>,
) {
    for memory in memories {
        if memory.memory_type == MemoryType::Commitment
            && matches!(
                memory.status,
                MemoryStatus::Candidate | MemoryStatus::Confirmed
            )
        {
            let due = memory
                .occurred_at
                .as_deref()
                .and_then(parse_rfc3339)
                .or_else(|| parse_rfc3339(&memory.created_at))
                .unwrap_or(now);
            out.push(Reminder {
                reminder_id: format!("commitment:{}", memory.id),
                reminder_type: ReminderType::Commitment,
                person_id: person.id,
                person_name: person.display_name.clone(),
                title: format!("你答应过 TA：{}", truncate(&memory.content, 30)),
                detail: Some(memory.content.clone()),
                due_at: format_rfc3339(due),
                days_overdue: None,
                days_until: None,
            });
        }
        // Important recent events (job change, startup, moving, ...).
        let important = matches!(memory.importance, Some(Importance::High | Importance::Core))
            && matches!(
                memory.memory_type,
                MemoryType::Temporal | MemoryType::Episodic
            )
            && memory.status == MemoryStatus::Confirmed;
        if important {
            let occurred = memory
                .occurred_at
                .as_deref()
                .and_then(parse_rfc3339)
                .or_else(|| parse_rfc3339(&memory.created_at));
            if let Some(occurred) = occurred {
                let age_days = (now - occurred).whole_days();
                if (0..=IMPORTANT_EVENT_WINDOW_DAYS).contains(&age_days) {
                    out.push(Reminder {
                        reminder_id: format!("event:{}", memory.id),
                        reminder_type: ReminderType::ImportantEvent,
                        person_id: person.id,
                        person_name: person.display_name.clone(),
                        title: format!("重要变化：{}", truncate(&memory.content, 30)),
                        detail: Some(memory.content.clone()),
                        due_at: format_rfc3339(occurred),
                        days_overdue: None,
                        days_until: None,
                    });
                }
            }
        }
    }
}

async fn append_story_anniversaries(
    store: &Arc<dyn MissoryStore>,
    scope: &MissoryScope,
    now: OffsetDateTime,
    out: &mut Vec<Reminder>,
) -> sdkwork_missory_spi::MissoryStoreResult<()> {
    let (stories, _) = store
        .list_stories(
            scope,
            ListStoriesQuery {
                page: Some(1),
                page_size: Some(200),
                ..ListStoriesQuery::default()
            },
        )
        .await?;
    for story in stories {
        let Some(started) = story.started_at.as_deref().and_then(parse_rfc3339) else {
            continue;
        };
        // Anniversary within the next lead window (or today).
        for offset in 0..=ANNIVERSARY_LEAD_DAYS {
            let day = now + time::Duration::days(offset);
            if day.month() == started.month() && day.day() == started.day() {
                let participants = story.participant_ids.len();
                out.push(Reminder {
                    reminder_id: format!("anniversary:{}", story.id),
                    reminder_type: ReminderType::Anniversary,
                    person_id: story.participant_ids.first().copied().unwrap_or_default(),
                    person_name: format!("《{}》", story.title),
                    title: format!(
                        "纪念日：距离《{}》已经过去 {} 年",
                        story.title,
                        { now.year() - started.year() }
                    ),
                    detail: Some(format!("参与人数 {participants}")),
                    due_at: format_rfc3339(day.date().with_time(time::Time::MIDNIGHT).assume_utc()),
                    days_overdue: None,
                    days_until: Some(offset),
                });
                break;
            }
        }
    }
    Ok(())
}

/// Applies dismiss/snooze state: dismissed keys are dropped, snoozed keys are
/// hidden until their snooze expires.
pub async fn filter_by_state(
    store: &Arc<dyn MissoryStore>,
    scope: &MissoryScope,
    reminders: Vec<Reminder>,
    now: OffsetDateTime,
) -> sdkwork_missory_spi::MissoryStoreResult<Vec<Reminder>> {
    let mut visible = Vec::with_capacity(reminders.len());
    for reminder in reminders {
        let state = store
            .get_reminder_state(scope, &reminder.reminder_id)
            .await?;
        match state {
            Some(ReminderStateRecord {
                status: ReminderStateStatus::Dismissed,
                ..
            }) => continue,
            Some(ReminderStateRecord {
                status: ReminderStateStatus::Snoozed,
                snoozed_until: Some(until),
                ..
            }) => {
                if let Some(until) = parse_rfc3339(&until) {
                    if until > now {
                        continue;
                    }
                }
            }
            _ => {}
        }
        visible.push(reminder);
    }
    Ok(visible)
}

/// Validates a reminder key shape for dismiss/snooze commands.
pub fn assert_valid_reminder_key(reminder_id: &str) -> MissoryServiceResult<()> {
    if reminder_id.is_empty()
        || !REMINDER_KEY_PREFIXES
            .iter()
            .any(|prefix| reminder_id.starts_with(prefix))
    {
        return Err(MissoryServiceError::invalid_parameter(
            "reminderId must look like '<type>:<id>' with a known type prefix",
        ));
    }
    Ok(())
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        value.to_string()
    } else {
        let cut: String = value.chars().take(max_chars).collect();
        format!("{cut}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_birthdays_when_parsing_then_month_day_is_returned() {
        assert_eq!(parse_birthday("05-20"), Some((5, 20)));
        assert_eq!(parse_birthday("1990-05-20"), Some((5, 20)));
        assert_eq!(parse_birthday("13-01"), None);
        assert_eq!(parse_birthday("abc"), None);
    }

    #[test]
    fn given_birthday_later_this_year_when_computing_next_occurrence_then_days_are_positive() {
        let today = time::Date::from_calendar_date(2026, time::Month::January, 10)
            .expect("date")
            .with_time(time::Time::MIDNIGHT)
            .assume_utc();
        let (instant, days_until) = next_birthday("05-20", today).expect("next birthday");
        assert_eq!(days_until, 130);
        assert_eq!(instant.month(), time::Month::May);
    }

    #[test]
    fn given_past_birthday_this_year_when_computing_next_occurrence_then_rolls_to_next_year() {
        let today = time::Date::from_calendar_date(2026, time::Month::June, 1)
            .expect("date")
            .with_time(time::Time::MIDNIGHT)
            .assume_utc();
        let (_, days_until) = next_birthday("05-20", today).expect("next birthday");
        assert_eq!(days_until, 353);
    }

    #[test]
    fn given_reminder_keys_when_validating_then_known_prefixes_pass() {
        assert!(assert_valid_reminder_key("birthday:123").is_ok());
        assert!(assert_valid_reminder_key("uncontacted:9").is_ok());
        assert!(assert_valid_reminder_key("bogus").is_err());
        assert!(assert_valid_reminder_key("").is_err());
    }
}
