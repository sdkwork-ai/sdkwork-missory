// IAM runtime seam (phase-2 adoption point, TECH_ARCHITECTURE section 8).
//
// The fleet pattern wraps the app-api router with the sdkwork-iam-web-adapter
// resolver (dual-token: Authorization: Bearer <auth> + Access-Token: <access>)
// and projects WebRequestPrincipal into the domain request context. The
// generated SDK client already accepts a tokenManager for that flow; standalone
// development resolves identity through the gateway's dev bypass until the IAM
// runtime (and its PostgreSQL session store) is provisioned for this app.
export const IAM_INTEGRATION_PHASE = 2;

export interface IamRuntimeDescriptor {
  appId: string;
  dualTokenHeaders: { auth: string; access: string };
}

export function describeIamRuntime(): IamRuntimeDescriptor {
  return {
    appId: "sdkwork-missory-pc",
    dualTokenHeaders: { auth: "Authorization", access: "Access-Token" },
  };
}
