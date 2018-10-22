use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::prelude::*;
use schema::posts::dsl::*;
use schema::posts;
use serde::Serialize;
use std::time::SystemTime;
use pg_pool::DbConn;
use diesel::result::Error;
use request::ArticleEditForm;
use chrono::prelude::*;

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
}

impl Post {
    pub fn load_all(include_unpublished: bool, conn: &DbConn) -> Vec<Post> {
        if include_unpublished {
            posts::table.load::<Post>(&**conn).expect("something wrong")
        } else {
            posts::table.filter(published.eq(true)).load::<Post>(&**conn).expect("something wrong")
        }
    }
    pub fn find(fetched_id: i32, conn: &DbConn) -> Result<Post, Error> {
        posts::table.find(fetched_id).first::<Post>(&**conn)
    }

//    pub fn new(article: ArticleEditForm) -> Post {
//    }

    pub fn form_article_edit_form(article: &ArticleEditForm, current_user_id: i32) -> Post {
        let timestamp = if article.publish_at.eq("") {
            Utc::now().timestamp()
        } else {
            NaiveDateTime::parse_from_str(&article.publish_at, "%Y-%m-%dT%H:%M").unwrap().timestamp()
        };

        Post {
            id: article.id.unwrap_or(-1),
            title: article.title.clone(),
            body: article.body.clone(),
            published: article.published,
            user_id: current_user_id,
            publish_at: NaiveDateTime::from_timestamp(timestamp, 0),
            url: article.url.clone(),
        }
    }
}

#[derive(Queryable, Debug, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_at: NaiveDateTime,
    pub last_login_at: NaiveDateTime,
}

impl User {
    pub fn authenticated(&self, password: &str) -> bool {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        let result = hasher.result_str();

        if self.password.eq(&result) {
            true
        } else {
            false
        }
    }
}
