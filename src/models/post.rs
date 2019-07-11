use crate::schema::categories;
use chrono::NaiveDateTime;
#[derive(Queryable, Debug, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub category_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Tag {
    pub id: i64,
    pub post_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
