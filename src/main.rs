use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener,TcpStream};
use std::env;


#[macro_use]
extern crate serde_derive;

// use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

const DB_URL: &str = !env("DATABASE_URL");


const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\Content-Type: application/json\r\n\r\n";
const CREATED_RESPONSE: &str = "HTTP/1.1 201 Created\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const BAD_REQUEST_RESPONSE: &str = "HTTP/1.1 400 Bad Request\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";


fn main() {
    if let Err(e) = set_database(){
        println!("Error setting database: {}", e);
        return;
    }

    let listener = TcpListener::bind(format!("127.0.0.1:8080")).unwrap();
    println!("Listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            },
            Err(e) => {
                println!("Error: {}", e);
            }
}
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer){
        Ok(size) => {
            request = String::from_utf8_lossy(&buffer[..size]).to_string();

            request.push_str(String::from_utf8_lossy(&buffer[size..]).as_ref());
            
            let (status_line, content) = match &*request {
                r if request_with("POST", "/users") => handle_post_request(r),
                r if request_with("GET", "/users/") => handle_get_request(r),
                r if request_with("GET", "/users") => handle_get_all_request(r),
                r if request_with("PUT", "/users/") => handle_put_request(r),
                r if request_with("GET", "/users") => handle_delete_request(r),
                _ => (NOT_FOUND_RESPONSE,"Not Found".to_string()),
            }

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }

        Err(e) => {
            println!("Error {}", e);
            return;
        }
    }
}

// CONTROLLERS
fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
    (Ok(user), Ok(mut client)) => {
        client.execute("INSERT INTO users (name, email) VALUES ($1, $2)", &[&user.name, &user.email]).unwrap();
        (OK_RESPONSE.to_string(),"User created".to_string())
    }
    _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(),"Error".to_string())
}
}

fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>, Client::connect(DB_URL, NoTls)) {
        match client.query("SELECT * FROM users WHERE id = $1", &[&id]) {
            Ok(rows) => {
                let user = User {
                    id: rows.get(0).get(0),
                    name: rows.get(0).get(1),
                    email: rows.get(0).get(2),
                };
                }
                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(),"Error".to_string())
    }
}

fn handle_get_all_request(request: &str) -> (String, String) {
    match (Client::connect(DB_URL, NoTls)) {
        Ok(mut client) => {
            let mut users = Vec::new();
            for row in client.query("SELECT * FROM users", &[]).unwrap().iter() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(),"Error".to_string())
    }
}

fn handle_put_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute("UPDATE users SET name = $1, email = $2 WHERE id = $3", &[&user.name, &user.email, &user.id]).unwrap();
            (OK_RESPONSE.to_string(),"User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(),"Error".to_string())
    }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {

            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                return (NOT_FOUND_RESPONSE.to_string(),"User not found".to_string());
            }

            (OK_RESPONSE.to_string(),"User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(),"Error".to_string())
    }


fn set_database() -> Result<(), PostgresError> {

    let mut client = Client::connect(DB_URL, NoTls)?;

    client.execute("CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL
    )", &[])?;

}

fn start_server(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on http://127.0.0.1:8080");
}

fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()

}

fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}