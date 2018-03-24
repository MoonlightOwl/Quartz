use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema {
    table! {
        quotes (rowid) {
            rowid -> Nullable<Integer>,
            text -> Text,
        }
    }
}

use self::schema::quotes;
use self::schema::quotes::dsl::{quotes as all_quotes};

#[table_name="quotes"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Quote {
    pub rowid: Option<i32>,
    pub text: String,
}

impl Quote {
    pub fn all(conn: &SqliteConnection) -> Vec<Quote> {
        all_quotes.order(quotes::rowid.desc()).load::<Quote>(conn).unwrap()
    }

    pub fn get_with_id(id: i32, conn: &SqliteConnection) -> QueryResult<Quote> {
        all_quotes.find(id).get_result::<Quote>(conn)
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_quotes.find(id)).execute(conn).is_ok()
    }
}
