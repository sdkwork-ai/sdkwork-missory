// H5 IAM runtime seam (phase-2 adoption point). Dual-token wire contract:
// Authorization: Bearer <auth> + Access-Token: <access>; the generated SDK
// client accepts the tokenManager for that flow.
export const IAM_INTEGRATION_PHASE = 2;

export interface IamRuntimeDescriptor {
  appId: string;
  dualTokenHeaders: { auth: string; access: string };
}

export function describeIamRuntime(): IamRuntimeDescriptor {
  return {
    appId: "sdkwork-missory-h5",
    dualTokenHeaders: { auth: "Authorization", access: "Access-Token" },
  };
}
