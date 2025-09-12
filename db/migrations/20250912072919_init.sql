-- migrate:up
CREATE TYPE locale AS ENUM('en', 'zh-tw');


CREATE TABLE IF NOT EXISTS interaction (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMPTZ DEFAULT now(),
  locale locale NOT NULL,
  user_agent TEXT,
  duration INTEGER DEFAULT 0
);


CREATE INDEX if NOT EXISTS interaction_created_at_idx ON interaction (created_at);


-- migrate:down
DROP TYPE IF EXISTS locale;


DROP TABLE IF EXISTS interaction;


DROP INDEX IF EXISTS interaction_created_at_idx;
