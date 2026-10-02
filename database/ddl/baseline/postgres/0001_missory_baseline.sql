-- SDKWork missory consolidated initialization baseline (postgres)
-- Application is in initialization state: full DDL lives here; migrations/ is
-- reserved for post-GA changes (baseline-plus-migrations strategy).
--
-- Standard columns per DATABASE_SPEC.md: id (snowflake BIGINT PK), uuid,
-- audit (created_at/updated_at), lifecycle (version), subject scope
-- (tenant_id, organization_id, user_id). Timestamps are TIMESTAMPTZ.

BEGIN;

CREATE TABLE IF NOT EXISTS missory_my_profile (
  id BIGINT PRIMARY KEY,
  uuid VARCHAR(64) NOT NULL,
  tenant_id BIGINT NOT NULL,
  organization_id BIGINT NOT NULL DEFAULT 0,
  user_id BIGINT NOT NULL,
  display_name VARCHAR(200) NOT NULL DEFAULT '',
  nickname VARCHAR(200),
  city VARCHAR(120),
  occupation VARCHAR(120),
  company VARCHAR(200),
  education VARCHAR(300),
  interests JSONB NOT NULL DEFAULT '[]'::jsonb,
  likes JSONB NOT NULL DEFAULT '[]'::jsonb,
  dislikes JSONB NOT NULL DEFAULT '[]'::jsonb,
  communication_style VARCHAR(300),
  bio TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  version BIGINT NOT NULL DEFAULT 0,
  CONSTRAINT missory_my_profile_owner_unique UNIQUE (tenant_id, user_id)
);

CREATE TABLE IF NOT EXISTS missory_person (
  id BIGINT PRIMARY KEY,
  uuid VARCHAR(64) NOT NULL,
  tenant_id BIGINT NOT NULL,
  organization_id BIGINT NOT NULL DEFAULT 0,
  user_id BIGINT NOT NULL,
  display_name VARCHAR(120) NOT NULL,
  aliases JSONB NOT NULL DEFAULT '[]'::jsonb,
  gender VARCHAR(40),
  birthday VARCHAR(10),
  city VARCHAR(120),
  title VARCHAR(160),
  company VARCHAR(200),
  avatar_url VARCHAR(500),
  tags JSONB NOT NULL DEFAULT '[]'::jsonb,
  interests JSONB NOT NULL DEFAULT '[]'::jsonb,
  preferences JSONB NOT NULL DEFAULT '[]'::jsonb,
  bio TEXT,
  contact_channels JSONB NOT NULL DEFAULT '{}'::jsonb,
  notes TEXT,
  last_contacted_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  version BIGINT NOT NULL DEFAULT 0,
  deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_missory_person_owner_updated
  ON missory_person (tenant_id, user_id, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_missory_person_name
  ON missory_person (tenant_id, user_id, display_name);

CREATE TABLE IF NOT EXISTS missory_relationship (
  id BIGINT PRIMARY KEY,
  uuid VARCHAR(64) NOT NULL,
  tenant_id BIGINT NOT NULL,
  organization_id BIGINT NOT NULL DEFAULT 0,
  user_id BIGINT NOT NULL,
  person_id BIGINT NOT NULL REFERENCES missory_person (id) ON DELETE CASCADE,
  relationship_types JSONB NOT NULL,
  started_at TIMESTAMPTZ,
  last_contacted_at TIMESTAMPTZ,
  description TEXT,
  importance VARCHAR(20),
  contact_cycle_days BIGINT,
  notes TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  version BIGINT NOT NULL DEFAULT 0,
  CONSTRAINT missory_relationship_person_unique UNIQUE (tenant_id, user_id, person_id)
);

CREATE TABLE IF NOT EXISTS missory_memory (
  id BIGINT PRIMARY KEY,
  uuid VARCHAR(64) NOT NULL,
  tenant_id BIGINT NOT NULL,
  organization_id BIGINT NOT NULL DEFAULT 0,
  user_id BIGINT NOT NULL,
  person_id BIGINT NOT NULL REFERENCES missory_person (id) ON DELETE CASCADE,
  story_id BIGINT,
  memory_type VARCHAR(30) NOT NULL,
  title VARCHAR(200),
  content TEXT NOT NULL,
  origin VARCHAR(20) NOT NULL DEFAULT 'fact',
  status VARCHAR(20) NOT NULL DEFAULT 'candidate',
  confidence DOUBLE PRECISION,
  source_reason TEXT,
  source_kind VARCHAR(30) NOT NULL DEFAULT 'user-input',
  source_ref VARCHAR(300),
  importance VARCHAR(20),
  occurred_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  version BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_missory_memory_owner_type_status
  ON missory_memory (tenant_id, user_id, memory_type, status);
CREATE INDEX IF NOT EXISTS idx_missory_memory_person_time
  ON missory_memory (tenant_id, user_id, person_id, occurred_at DESC);

CREATE TABLE IF NOT EXISTS missory_story (
  id BIGINT PRIMARY KEY,
  uuid VARCHAR(64) NOT NULL,
  tenant_id BIGINT NOT NULL,
  organization_id BIGINT NOT NULL DEFAULT 0,
  user_id BIGINT NOT NULL,
  title VARCHAR(200) NOT NULL,
  summary TEXT,
  summary_origin VARCHAR(20),
  participant_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
  memory_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
  started_at TIMESTAMPTZ,
  ended_at TIMESTAMPTZ,
  location VARCHAR(200),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  version BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_missory_story_owner
  ON missory_story (tenant_id, user_id, started_at DESC);

CREATE TABLE IF NOT EXISTS missory_reminder_state (
  id BIGINT PRIMARY KEY,
  tenant_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  reminder_id VARCHAR(120) NOT NULL,
  status VARCHAR(20) NOT NULL,
  snoozed_until TIMESTAMPTZ,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT missory_reminder_state_key_unique UNIQUE (tenant_id, user_id, reminder_id)
);

COMMIT;
