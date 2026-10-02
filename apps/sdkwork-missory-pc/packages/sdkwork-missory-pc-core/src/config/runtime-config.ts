export type MissoryEnvironment = "development" | "test" | "staging" | "demo" | "production";
export type MissoryDeploymentProfile = "standalone" | "cloud";

export interface MissoryPcRuntimeConfig {
  environment: MissoryEnvironment;
  deploymentProfile: MissoryDeploymentProfile;
  profileId: string;
  runtimeTarget: "browser";
  browserOriginMode: "same-origin" | "cross-origin";
  defaultLocale: string;
  fallbackLocale: string;
  supportedLocales: string[];
  appApiBaseUrl: string;
  backendApiBaseUrl: string;
  openApiBaseUrl: string;
}

const ENVIRONMENTS: MissoryEnvironment[] = ["development", "test", "staging", "demo", "production"];
const PROFILES: MissoryDeploymentProfile[] = ["standalone", "cloud"];

function requireText(value: unknown, field: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`runtime-env field ${field} is missing`);
  }
  return value;
}

function requireHttpUrl(value: unknown, field: string): string {
  const text = requireText(value, field);
  const parsed = new URL(text);
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error(`runtime-env field ${field} must be an http(s) URL`);
  }
  if (
    (parsed.hostname === "127.0.0.1" || parsed.hostname === "localhost") &&
    process.env.NODE_ENV === "production"
  ) {
    throw new Error(`runtime-env field ${field} must not point at loopback in production`);
  }
  return text.replace(/\/+$/, "");
}

export function parseMissoryPcRuntimeConfig(input: unknown): MissoryPcRuntimeConfig {
  if (!input || typeof input !== "object") {
    throw new Error("runtime-env document is not an object");
  }
  const doc = input as Record<string, unknown>;
  const environment = requireText(doc.environment, "environment") as MissoryEnvironment;
  const deploymentProfile = requireText(doc.deploymentProfile, "deploymentProfile") as MissoryDeploymentProfile;
  if (!ENVIRONMENTS.includes(environment)) {
    throw new Error(`runtime-env environment is invalid: ${environment}`);
  }
  if (!PROFILES.includes(deploymentProfile)) {
    throw new Error(`runtime-env deploymentProfile is invalid: ${deploymentProfile}`);
  }
  const originMode = requireText(doc.browserOriginMode, "browserOriginMode");
  if (originMode !== "same-origin" && originMode !== "cross-origin") {
    throw new Error(`runtime-env browserOriginMode is invalid: ${originMode}`);
  }
  return {
    environment,
    deploymentProfile,
    profileId: requireText(doc.profileId, "profileId"),
    runtimeTarget: "browser",
    browserOriginMode: originMode,
    defaultLocale: requireText(doc.defaultLocale, "defaultLocale"),
    fallbackLocale: requireText(doc.fallbackLocale, "fallbackLocale"),
    supportedLocales: Array.isArray(doc.supportedLocales) ? (doc.supportedLocales as string[]) : ["zh-CN"],
    appApiBaseUrl: requireHttpUrl(doc.appApiBaseUrl, "appApiBaseUrl"),
    backendApiBaseUrl: requireHttpUrl(doc.backendApiBaseUrl, "backendApiBaseUrl"),
    openApiBaseUrl: requireHttpUrl(doc.openApiBaseUrl, "openApiBaseUrl"),
  };
}

export async function loadMissoryPcRuntimeConfig(
  fetcher: typeof fetch = fetch,
): Promise<MissoryPcRuntimeConfig> {
  const response = await fetcher("/runtime-env.json", { cache: "no-store", credentials: "same-origin" });
  if (!response.ok) {
    throw new Error(`Runtime configuration failed with HTTP ${response.status}`);
  }
  return parseMissoryPcRuntimeConfig(await response.json());
}
