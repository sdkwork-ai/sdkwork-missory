import type { Int64Id } from './int64-id';
import type { MissoryReminderType } from './missory-reminder-type';

export interface MissoryReminder {
  reminderId: string;
  type: MissoryReminderType;
  personId: Int64Id;
  personName?: string;
  title: string;
  detail?: string;
  dueAt: string;
  daysOverdue?: number;
  daysUntil?: number;
}
