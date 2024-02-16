CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS "users" CASCADE;

DROP TABLE IF EXISTS "individuals" CASCADE;

DROP TYPE IF EXISTS "relationship_types";

DROP TABLE IF EXISTS "relationships" CASCADE;

CREATE TABLE "users" (
  "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
  "email" VARCHAR(255) UNIQUE NOT NULL,
  "password" VARCHAR(255) NOT NULL,
  "is_admin" BOOLEAN DEFAULT false,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id")
);

CREATE TABLE "individuals" (
  "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
  "name" VARCHAR(255) NOT NULL,
  "image" VARCHAR(255),
  "birthday" TIMESTAMPTZ,
  "generation" INTEGER NOT NULL,
  "is_alive" BOOLEAN NOT NULL,
  "death" TIMESTAMPTZ,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id")
);

CREATE TYPE "relationship_types" AS ENUM (
  'PARENT_SON',
  'MARRIAGE',
  'DIVORCE'
);

CREATE TABLE "relationships" (
  "type" "relationship_types" NOT NULL,
  "from_id" UUID NOT NULL,
  "to_id" UUID NOT NULL,
  FOREIGN KEY ("from_id") REFERENCES "individuals"("id") ON DELETE CASCADE,
  FOREIGN KEY ("to_id") REFERENCES "individuals"("id") ON DELETE CASCADE,
  PRIMARY KEY ("from_id", "to_id")
);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE TRIGGER update_user_updated_at
BEFORE UPDATE ON "users"
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at();

CREATE OR REPLACE TRIGGER update_individual_updated_at
BEFORE UPDATE ON "individuals"
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at();
