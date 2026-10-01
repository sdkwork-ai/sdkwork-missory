import type { Int64Id } from './int64-id';

/** Resolved request identity (never client-writable at create/update bodies). */
export interface MissoryPersonRequestContext {
  tenantId: Int64Id;
  organizationId?: Int64Id;
  userId: Int64Id;
}
