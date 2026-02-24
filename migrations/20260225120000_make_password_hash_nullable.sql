-- Make password_hash optional (for email/password and future Google/GitHub login)
ALTER TABLE "user" ALTER COLUMN password_hash DROP NOT NULL;
