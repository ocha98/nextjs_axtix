-- Your SQL goes here

CREATE TABLE "user" (
    -- user_idは英数字とアンダースコアのみ
    "user_id" VARCHAR(20) CHECK (user_id ~ '^[A-Za-z0-9_]+$') PRIMARY KEY,
    "displayname" VARCHAR(20) NOT NULL,
    "bio" VARCHAR(200) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "prof_img_url" VARCHAR(200) NOT NULL
);

-- update_atを自動更新するもの
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- トリガーの定義
CREATE TRIGGER update_user_updated_at
BEFORE UPDATE ON "user"
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- OAuthのプロバイダー
CREATE TYPE oauth_provider AS ENUM ('GitHub', 'Google', 'Facebook');
CREATE TABLE "oauth" (
    "id" SERIAL PRIMARY KEY,
    "user_id" VARCHAR(20) REFERENCES "user" (user_id),
    "provider" oauth_provider NOT NULL,
    "provider_user_id" VARCHAR(100) NOT NULL,
    UNIQUE ("provider", "provider_user_id"),
    UNIQUE ("user_id", "provider")
)

