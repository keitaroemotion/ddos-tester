extern crate postgres;
use postgres::{Connection, TlsMode};
use tokio_postgres::types::ToSql;

pub struct Database {
   conn: Connection,
}

impl       Database {
    fn set_connection() -> Connection {
        return Connection::connect(
                   "postgres://moomin@localhost:5432",
                   TlsMode::None
               )
               .unwrap();
    }

    fn execute(&self, query: &str, params: &[&dyn ToSql]) {
        self.conn
            .execute(query, &params)
            .unwrap();
    }

    fn create_table(&self, table_name: &str) {
        self.execute("", &[]);
    }
}

pub fn create_table(_db: &Database, _table_name: &str, _field_pairs: Vec<Vec<&str>>) {
    let mut query = format!("CREATE TABLE IF NOT EXISTS {} (\n", _table_name).to_string();
    for row in _field_pairs {
        for r in row {
            query.push_str(&format!("    {} ", r.to_string()));
        }
        query.push_str(&",\n".to_string());
    }
    query.truncate(query.len() - 2);
    query.push_str(&"\n)".to_string());
    println!("{}", &query);

    _db.execute(&query, &[]);
}

mod Type {
    pub const BYTEA              :&str = "BYTEA";
    pub const VARCHAR            :&str = "VARCHAR";
    pub const SERIAL_PRIMARY_KEY :&str = "VARCHAR";
    pub const NOT_NULL           :&str = "NOT NULL";
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Person {
        id:   i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    #[test]
    fn test_create_table()
    {
        let table = "moomin";
        let db    = Database { conn: Database::set_connection() };

        create_table(
            &db,
            &table,
            vec!
            [
                vec!["id",   Type::SERIAL_PRIMARY_KEY     ],
                vec!["name", Type::VARCHAR, Type::NOT_NULL],
                vec!["data", Type::BYTEA                  ],
            ]
        );

        let me = Person {
                     id:   0,
                     name: "Steven".to_string(),
                     data: None,
                 };

        db.execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&me.name, &me.data]
        );

        for row in &db.conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
            let person = Person {
                             id:   row.get(0),
                             name: row.get(1),
                             data: row.get(2),
                         };
            //println!("Found person {}: {}", person.id, person.name);
        }
    }
}
