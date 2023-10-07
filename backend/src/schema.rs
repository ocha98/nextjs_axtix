// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "oauth_provider"))]
    pub struct OauthProvider;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OauthProvider;

    oauth (id) {
        id -> Int4,
        #[max_length = 20]
        user_id -> Nullable<Varchar>,
        provider -> OauthProvider,
        #[max_length = 100]
        provider_user_id -> Varchar,
    }
}

diesel::table! {
    user (user_id) {
        #[max_length = 20]
        user_id -> Varchar,
        #[max_length = 20]
        displayname -> Varchar,
        #[max_length = 200]
        bio -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 200]
        prof_img_url -> Varchar,
    }
}

diesel::joinable!(oauth -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    oauth,
    user,
);
