table! {
    accounts (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        nickname -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        intro -> Text,
        permission -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        author_id -> Uuid,
        category_id -> Uuid,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        post_id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(posts -> accounts (author_id));
joinable!(posts -> categories (category_id));
joinable!(tags -> posts (post_id));

allow_tables_to_appear_in_same_query!(accounts, categories, posts, tags,);
