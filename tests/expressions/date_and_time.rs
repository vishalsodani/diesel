use schema::connection;
use yaqb::*;
use yaqb::types::structs::*;
use yaqb::expression::dsl::*;

table! {
    has_timestamps {
        id -> Serial,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    has_time {
        id -> Serial,
        time -> Time,
    }
}

#[test]
fn now_executes_sql_function_now() {
    use self::has_timestamps::dsl::*;

    let connection = connection();
    setup_test_table(&connection);
    connection.execute("INSERT INTO has_timestamps (created_at) VALUES
                       (NOW() - '1 day'::interval), (NOW() + '1 day'::interval)")
        .unwrap();

    let before_today: Vec<i32> = has_timestamps.select(id)
        .filter(created_at.lt(now))
        .load(&connection)
        .unwrap().collect();
    let after_today: Vec<i32> = has_timestamps.select(id)
        .filter(created_at.gt(now))
        .load(&connection)
        .unwrap().collect();
    assert_eq!(vec![1], before_today);
    assert_eq!(vec![2], after_today);
}

#[test]
fn date_uses_sql_function_date() {
    use self::has_timestamps::dsl::*;

    let connection = connection();
    setup_test_table(&connection);
    connection.execute("INSERT INTO has_timestamps (created_at, updated_at) VALUES
                       ('2015-11-15 06:07:41', '2015-11-15 20:07:41'),
                       ('2015-11-16 06:07:41', '2015-11-17 20:07:41'),
                       ('2015-11-16 06:07:41', '2015-11-16 02:07:41')
                       ").unwrap();

    let expected_data = vec![1, 3];
    let actual_data: Vec<_> = has_timestamps.select(id)
        .filter(date(created_at).eq(date(updated_at)))
        .load(&connection)
        .unwrap().collect();
    assert_eq!(expected_data, actual_data);
}

#[test]
fn time_is_deserialized_properly() {
    use self::has_time::dsl::*;

    let connection = connection();
    setup_test_table(&connection);
    connection.execute("INSERT INTO has_time (\"time\") VALUES
                       ('00:00:01'), ('00:02:00'), ('03:00:00')
                       ").unwrap();
    let one_second = PgTime(1_000_000);
    let two_minutes = PgTime(120_000_000);
    let three_hours = PgTime(10_800_000_000);
    let expected_data = vec![one_second, two_minutes, three_hours];

    let actual_data: Vec<_> = has_time.select(time)
        .load(&connection)
        .unwrap().collect();
    assert_eq!(expected_data, actual_data);
}


fn setup_test_table(conn: &Connection) {
    conn.execute("CREATE TABLE has_timestamps (
        id SERIAL PRIMARY KEY,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )").unwrap();
    conn.execute("CREATE TABLE has_time (
        id SERIAL PRIMARY KEY,
        \"time\" TIME NOT NULL
    )").unwrap();
}