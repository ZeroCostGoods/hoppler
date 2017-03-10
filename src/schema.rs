table! {
    events {
        id -> Integer,
        timestamp -> BigInt,
        session_id -> VarChar,
        time_on_page -> Nullable<Integer>,
        username -> VarChar,
        event_type -> VarChar,
        hostname -> VarChar,
        pathname -> VarChar,
        search -> Nullable<VarChar>,
        in_focus -> Bool,
        prior_hostname -> Nullable<VarChar>,
        prior_pathname -> Nullable<VarChar>,
        prior_search -> Nullable<VarChar>,
    }
}