

fn conn_init(db_name: &str) -> sqlite::Connection {
    // create a connection to the database
    let connection = sqlite::open(&db_name).unwrap();
    // ensure the schema exists
    connection.execute(" CREATE TABLE IF NOT EXISTS
        store (key TEXT PRIMARY KEY, value TEXT);",).unwrap();
    // return the connection
    connection

}
pub fn set(db_name: &str, key: &str, value: &str) {
    // create or update a key, value pair
    let connection = conn_init(&db_name);
    let expr = format!("INSERT OR REPLACE INTO store
        VALUES ('{}', '{}')", key, value);
    connection.execute(expr).unwrap();

}


pub fn get(db_name: &str, key: &str) -> String {
    // get the value for a key
    let connection = conn_init(&db_name);
    let expr = format!("SELECT value FROM store WHERE key = '{}'", key);
    let mut statement = connection.prepare(expr).unwrap();
    // this is a "loop" through results, even though there should be one result at most
    let mut value: String = String::new(); // this blank string will be return if no key is found
    while let sqlite::State::Row = statement.next().unwrap() {
        value =  statement.read::<String>(0).unwrap();
    }
    // return the value
    value
}

pub fn delete(db_name: &str, key: &str) {
    // delete a key
    let connection = conn_init(&db_name);
    let expr = format!("DELETE FROM store WHERE key = '{}'", key);
    connection.execute(expr).unwrap();
}


pub fn list_keys(db_name: &str, ) -> Vec<String> {
    // return a list of keys as a Vec<String>
    let mut key: String;// = String::new();
    let mut keys =  Vec::new();
    let connection = conn_init(&db_name);
    let expr = String::from("SELECT key FROM store");
    let mut statement = connection.prepare(expr).unwrap();
    // loop over the keys, pushing them onto the vector
    while let sqlite::State::Row = statement.next().unwrap() {
        key =  statement.read::<String>(0).unwrap();
        keys.push(key);
    }
    // return the keys
    keys
}



#[test]
// use zoea::kv_database::{get, set, delete, list_keys}
fn readme_demo() {
    let db: &str = "TestingDatabase"; // can be &str or &String
    let key = "Key1"; // can be &str or &String
    let value = "Value you want to insert"; // can be &str or &String
    // SET the value of key in database db
    set(&db, &key, &value);
    // GET the value of key in database db
    let same_value = get(&db, &key);
    assert_eq!(String::from(value), same_value);
    // LIST keys in database db
    let keys = list_keys(&db);
    // GETting an empty key returns an empty string
    let non_key = "KeyThatDoesNotExist";
    let non_value = get(&db, &non_key); // returns String::new();
    // DELETE a key in database db
    delete(&db, &key);
}