-- Enable UUID extension (if not already enabled)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- user
CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    mobile VARCHAR(32) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(30),
    avatar UUID,
    header UUID,
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ
);

-- media (referenced by user.avatar, user.header, and post_media)
CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type VARCHAR(32),
    path VARCHAR(512)
);

-- Add FKs for user avatar/header -> media
ALTER TABLE "user"
    ADD CONSTRAINT user_avatar_fk FOREIGN KEY (avatar) REFERENCES media(id),
    ADD CONSTRAINT user_header_fk FOREIGN KEY (header) REFERENCES media(id);

-- follow
CREATE TABLE follow (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    following_user_id UUID NOT NULL REFERENCES "user"(id),
    followed_user_id UUID NOT NULL REFERENCES "user"(id),
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ
);

-- post
CREATE TABLE post (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    body TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES "user"(id),
    reply_to_post_id UUID REFERENCES post(id),
    quote_to_post_id UUID REFERENCES post(id),
    status VARCHAR(32) NOT NULL DEFAULT 'published',
    deleted BOOLEAN NOT NULL DEFAULT false,
    edited_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ
);

-- like
CREATE TABLE "like" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id UUID NOT NULL REFERENCES post(id),
    user_id UUID NOT NULL REFERENCES "user"(id),
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ
);

-- repost
CREATE TABLE repost (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id UUID NOT NULL REFERENCES post(id),
    user_id UUID NOT NULL REFERENCES "user"(id),
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ
);

-- post_media (media_id references media.id)
CREATE TABLE post_media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id UUID NOT NULL REFERENCES post(id),
    media_id UUID REFERENCES media(id)
);

-- Indexes for common lookups
CREATE INDEX idx_follow_following ON follow(following_user_id);
CREATE INDEX idx_follow_followed ON follow(followed_user_id);
CREATE INDEX idx_like_post ON "like"(post_id);
CREATE INDEX idx_like_user ON "like"(user_id);
CREATE INDEX idx_post_user ON post(user_id);
CREATE INDEX idx_post_reply ON post(reply_to_post_id);
CREATE INDEX idx_repost_post ON repost(post_id);
CREATE INDEX idx_repost_user ON repost(user_id);
CREATE INDEX idx_post_media_post ON post_media(post_id);