table! {
    movie_details (id) {
        id -> Integer,
        uuid -> Text,
        thumb -> Text,
        icon -> Text,
        description -> Text,
    }
}

table! {
    movie_filelocations (id) {
        id -> Integer,
        uuid -> Text,
        movie -> Text,
        title -> Text,
        epi -> Integer,
        description -> Text,
        filename -> Text,
        thumb -> Text,
    }
}

table! {
    movie_progress (id) {
        id -> Integer,
        uuid -> Text,
        progress -> Integer,
        user -> Text,
    }
}

table! {
    movie_relationships (id) {
        id -> Integer,
        uuid_arr -> Text,
    }
}

table! {
    movie_watchtime (id) {
        id -> Integer,
        user -> Text,
        watchtime -> Integer,
    }
}

table! {
    movies (id) {
        id -> Integer,
        uuid -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        titles -> Text,
        categories -> Text,
        age_restriction -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    movie_details,
    movie_filelocations,
    movie_progress,
    movie_relationships,
    movie_watchtime,
    movies,
);
