

fn conn_init(db_name: &String) -> sqlite::Connection {
    // establish a new connection to the database
    let file: String = format!("{}.db", &db_name);
    let connection = sqlite::open(&file).unwrap();
    // ensure the table exists
    connection.execute(" CREATE TABLE IF NOT EXISTS
        store (key TEXT PRIMARY KEY, value TEXT);",).unwrap();
    // return the connection
    connection
}


pub fn set(db_name: &String, key: &String, value: &String) {
    // create or update a key, value pair
    let connection = conn_init(db_name);
    let expr = format!("INSERT OR REPLACE INTO store
        VALUES ('{}', '{}')", key, value);
    connection.execute(expr).unwrap();
}


pub fn get(db_name: &String, key: &String) -> String {
    // get the value for a key
    let connection = conn_init(db_name);
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

pub fn delete(db_name: &String, key: &String) {
    // delete a key
    let connection = conn_init(db_name);
    let expr = format!("DELETE FROM store WHERE key = '{}'", key);
    connection.execute(expr).unwrap();
}


pub fn list_keys(db_name: &String, ) -> Vec<String> {
    // return a list of keys as a Vec<String>
    let mut key: String;// = String::new();
    let mut keys =  Vec::new();
    let connection = conn_init(db_name);
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
