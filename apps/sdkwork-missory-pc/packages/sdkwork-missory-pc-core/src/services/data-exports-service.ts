import type { MissoryDataExport, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createDataExportsService(client: SdkworkAppClient) {
  return {
    exportData(): Promise<MissoryDataExport> {
      return client.missory.dataExports.create({});
    },
  };
}
