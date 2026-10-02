export function formatRelativeDays(days: number | undefined | null): string {
  if (days === undefined || days === null) return "暂无记录";
  if (days <= 0) return "今天";
  if (days < 30) return `${days} 天前`;
  if (days < 365) return `${Math.floor(days / 30)} 个月前`;
  return `${Math.floor(days / 365)} 年前`;
}

export function truncate(text: string, max: number): string {
  return text.length <= max ? text : `${text.slice(0, max)}…`;
}

export function formatDateTime(value: string | undefined | null): string {
  if (!value) return "—";
  return value.slice(0, 10);
}
