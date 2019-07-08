CREATE TABLE "public"."accounts"
(
    "id"         SERIAL8      NOT NULL PRIMARY KEY,
    "email"      VARCHAR(255) NOT NULL UNIQUE,
    "password"   VARCHAR(255) NOT NULL,
    "nickname"   VARCHAR(255) NOT NULL DEFAULT '',
    "avatar"     VARCHAR(255) NOT NULL DEFAULT '',
    "intro"      TEXT         NOT NULL DEFAULT '',
    "permission" INT2         NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ
);
SELECT diesel_manage_updated_at('accounts');
COMMENT ON TABLE "public"."accounts" IS '账户';
COMMENT ON COLUMN "public"."accounts"."id" IS '账户ID';
COMMENT ON COLUMN "public"."accounts"."email" IS 'email';
COMMENT ON COLUMN "public"."accounts"."password" IS '密码';
COMMENT ON COLUMN "public"."accounts"."nickname" IS '昵称';
COMMENT ON COLUMN "public"."accounts"."avatar" IS '头像';
COMMENT ON COLUMN "public"."accounts"."intro" IS '自我介绍';
COMMENT ON COLUMN "public"."accounts"."permission" IS '权限';


CREATE TABLE "public"."categories"
(
    "id"         SERIAL8      NOT NULL PRIMARY KEY,
    "name"       VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ
);
SELECT diesel_manage_updated_at('categories');
COMMENT ON TABLE "public"."categories" IS '文章分类';
COMMENT ON COLUMN "public"."categories"."id" IS '分类ID';
COMMENT ON COLUMN "public"."categories"."name" IS '分类名称';


CREATE TABLE "public"."posts"
(
    "id"          SERIAL8      NOT NULL PRIMARY KEY,
    "author_id"   SERIAL8      NOT NULL REFERENCES public.accounts (id),
    "category_id" SERIAL8      NOT NULL REFERENCES public.categories (id),
    "title"       VARCHAR(255) NOT NULL,
    "content"     TEXT         NOT NULL DEFAULT '',
    "created_at"  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at"  TIMESTAMPTZ
);
SELECT diesel_manage_updated_at('posts');
COMMENT ON TABLE "public"."posts" IS '文章';
COMMENT ON COLUMN "public"."posts"."id" IS '文章ID';
COMMENT ON COLUMN "public"."posts"."author_id" IS '关联作者';
COMMENT ON COLUMN "public"."posts"."category_id" IS '关联文章分类';
COMMENT ON COLUMN "public"."posts"."title" IS '文章标题';
COMMENT ON COLUMN "public"."posts"."content" IS '文章内容';


CREATE TABLE "public"."tags"
(
    "id"         SERIAL8      NOT NULL PRIMARY KEY,
    "post_id"    SERIAL8      NOT NULL REFERENCES public.posts (id),
    "name"       VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ
);
SELECT diesel_manage_updated_at('tags');
COMMENT ON TABLE "public"."tags" IS '标签';
COMMENT ON COLUMN "public"."tags"."id" IS '标签ID';
COMMENT ON COLUMN "public"."tags"."post_id" IS '关联文章';
COMMENT ON COLUMN "public"."tags"."name" IS '标签名称';
