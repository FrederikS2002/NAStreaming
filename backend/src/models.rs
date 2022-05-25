use super::schema::*;

#[derive(Queryable, Debug)]
pub struct Movie {
    id: i32,
    uuid: String,
    type_: String,
    titles: String,
    categories: String,
    age_restriction: i32,
    img_src: String,

}

#[derive(Insertable, Debug)]
#[table_name="movies"]
pub struct NewMovie {
    uuid: String,
    type_: String,
    titles: String,
    categories: String,
    age_restriction: i32,
    img_src: String,

}
