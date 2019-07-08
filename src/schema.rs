table! {
    accounts (id) {
        id -> Int8,
        email -> Varchar,
        password -> Varchar,
        nickname -> Varchar,
        avatar -> Varchar,
        intro -> Text,
        permission -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    categories (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Int8,
        author_id -> Int8,
        category_id -> Int8,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tags (id) {
        id -> Int8,
        post_id -> Int8,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(posts -> accounts (author_id));
joinable!(posts -> categories (category_id));
joinable!(tags -> posts (post_id));

allow_tables_to_appear_in_same_query!(accounts, categories, posts, tags,);
