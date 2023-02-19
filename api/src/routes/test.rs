use std::error::Error;

use sqlx::{decode::Decode, postgres::PgValueRef, types::Type, Postgres};

pub struct Seller {
    //username: String,
    photo: Option<String>,
}

impl<'r> Decode<'r, Postgres> for Seller
where
    //String: Decode<'r, Postgres>,
    String: Type<Postgres>,
    //Option<String>: Decode<'r, Postgres>,
    Option<String>: Type<Postgres>,
{
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        //let username = decoder.try_decode::<String>()?;
        let photo = decoder.try_decode::<Option<String>>()?;
        Ok(Seller {
            // username
            photo,
        })
    }
}
