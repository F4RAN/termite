-- Make nickname required (NOT NULL)
-- Ensure existing rows have a nickname before running (e.g. UPDATE "user" SET nickname = username WHERE nickname IS NULL;)
ALTER TABLE "user" ALTER COLUMN nickname SET NOT NULL;
