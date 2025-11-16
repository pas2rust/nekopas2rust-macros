#![cfg(feature = "sql")]
use nekopas2rust_macros::Sql;

#[derive(Sql)]
struct User {
    uuid: &'static str,
    name: String,
    nick: &'static str,
}

#[test]
fn sql_basic() {
    let user = User {
        uuid: "1",
        name: "John".to_string(),
        nick: "Doe",
    };

    let (insert_sql, insert_params) = user.sql_insert();
    assert_eq!(
        insert_sql,
        "INSERT INTO user (uuid, name, nick) VALUES ($1, $2, $3)"
    );
    assert_eq!(insert_params.len(), 3);

    let (select_all_sql, select_all_params) = user.sql_select_all();
    assert_eq!(select_all_sql, "SELECT uuid, name, nick FROM user");
    assert_eq!(select_all_params.len(), 0);
}

#[test]
fn sql_per_field_helpers() {
    let user = User {
        uuid: "1",
        name: "John".to_string(),
        nick: "Doe",
    };

    let (s_name_sql, s_name_params) = user.sql_select_by_name();
    assert_eq!(
        s_name_sql,
        "SELECT uuid, name, nick FROM user WHERE name = $1"
    );
    assert_eq!(s_name_params.len(), 1);

    let (d_name_sql, d_name_params) = user.sql_delete_by_name();
    assert_eq!(d_name_sql, "DELETE FROM user WHERE name = $1");
    assert_eq!(d_name_params.len(), 1);

    let (u_name_sql, u_name_params) = user.sql_update_by_name();
    assert_eq!(
        u_name_sql,
        "UPDATE user SET uuid = $1, nick = $2 WHERE name = $3"
    );
    assert_eq!(u_name_params.len(), 3);

    let (s_nick_sql, s_nick_params) = user.sql_select_by_nick();
    assert_eq!(
        s_nick_sql,
        "SELECT uuid, name, nick FROM user WHERE nick = $1"
    );
    assert_eq!(s_nick_params.len(), 1);

    let (d_nick_sql, d_nick_params) = user.sql_delete_by_nick();
    assert_eq!(d_nick_sql, "DELETE FROM user WHERE nick = $1");
    assert_eq!(d_nick_params.len(), 1);

    let (u_nick_sql, u_nick_params) = user.sql_update_by_nick();
    assert_eq!(
        u_nick_sql,
        "UPDATE user SET uuid = $1, name = $2 WHERE nick = $3"
    );
    assert_eq!(u_nick_params.len(), 3);

    let (s_uuid_sql, s_uuid_params) = user.sql_select_by_uuid();
    assert_eq!(
        s_uuid_sql,
        "SELECT uuid, name, nick FROM user WHERE uuid = $1"
    );
    assert_eq!(s_uuid_params.len(), 1);

    let (d_uuid_sql, d_uuid_params) = user.sql_delete_by_uuid();
    assert_eq!(d_uuid_sql, "DELETE FROM user WHERE uuid = $1");
    assert_eq!(d_uuid_params.len(), 1);

    let (u_uuid_sql, u_uuid_params) = user.sql_update_by_uuid();
    assert_eq!(
        u_uuid_sql,
        "UPDATE user SET name = $1, nick = $2 WHERE uuid = $3"
    );
    assert_eq!(u_uuid_params.len(), 3);
}
