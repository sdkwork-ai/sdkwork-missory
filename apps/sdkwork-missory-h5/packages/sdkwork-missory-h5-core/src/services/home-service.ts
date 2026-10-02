import type { SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createHomeService(client: SdkworkAppClient) {
  return {
    today() {
      return client.missory.home.today.list();
    },
    reminders(params: { page?: number; pageSize?: number; type?: string } = {}) {
      return client.missory.reminders.list({
        page: params.page,
        pageSize: params.pageSize,
        type_: params.type as never,
      });
    },
    dismissReminder(reminderId: string) {
      return client.missory.reminders.dismiss(reminderId);
    },
    snoozeReminder(reminderId: string, days: number) {
      return client.missory.reminders.snooze(reminderId, { days });
    },
  };
}
