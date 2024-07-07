use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::schema;

use super::models::Message;

pub fn load_messages_from_db(
    conn: &mut diesel::SqliteConnection
) -> Result<Vec<Message>, diesel::result::Error> {
    schema::message::table.load::<Message>(conn)
}

pub fn insert_message_to_db(
    conn: &mut diesel::SqliteConnection, 
    timestamp: String,
    content: String,
) -> Result<Message, diesel::result::Error> {
    diesel::insert_into(schema::message::table)
        .values((
            schema::message::timestamp.eq(timestamp),
            schema::message::content.eq(content)
        ))
        .returning(Message::as_returning())
        .get_result(conn)
}