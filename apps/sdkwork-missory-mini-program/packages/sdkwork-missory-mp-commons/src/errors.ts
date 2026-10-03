/**
 * Normalizes a thrown value crossing the SDK/host boundary into display text.
 * Pages show this through `note`/`error` view state; the fallback keeps the
 * rendered copy stable when a platform failure carries no message.
 */
export function resolveErrorMessage(error: unknown, fallback: string): string {
  if (error instanceof Error && error.message.trim().length > 0) {
    return error.message;
  }
  if (typeof error === "object" && error !== null && "message" in error) {
    const message = (error as { message?: unknown }).message;
    if (typeof message === "string" && message.trim().length > 0) {
      return message;
    }
  }
  if (typeof error === "string" && error.trim().length > 0) {
    return error;
  }
  return fallback;
}
