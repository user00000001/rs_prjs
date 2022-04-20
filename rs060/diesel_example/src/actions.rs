use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_uid(
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;
    Ok(user)
}

pub fn insert_new_user(
    nm: &str,
    conn: &SqliteConnection,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = models::User{
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    Ok(new_user)
}