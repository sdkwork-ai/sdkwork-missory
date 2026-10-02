import type { MissoryMyProfileUpsertRequest, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createProfileService(client: SdkworkAppClient) {
  return {
    retrieve() {
      return client.missory.myProfile.retrieve();
    },
    update(request: MissoryMyProfileUpsertRequest) {
      return client.missory.myProfile.update(request);
    },
  };
}
