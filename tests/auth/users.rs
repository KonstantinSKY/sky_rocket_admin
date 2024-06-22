// use fake::{Dummy, Fake, Faker};
use fake::{faker::internet::en::{Username, SafeEmail}, Fake};

use reqwest::{blocking::Client, header, StatusCode};
use serde_json::{json, Value};




const APP_HOST: &str = "http://127.0.0.1:8000";

#[test]

pub fn test_get_all_users() {
    // Setup
    let client = Client::new();
    // let rustacean1 = common::create_test_rustacean(&client);
    // let rustacean2 = common::create_test_rustacean(&client);

    // Test
    let response = client.get(format!("{}/auth/users", APP_HOST)).send().unwrap();
    assert_eq!( response.status(), StatusCode::OK );
    
    println!("Response: {:?}", response);
    // let response_text = response.text().unwrap();
    // println!("Response text: {:?}", response_text);
    let response_json: Value = response.json().unwrap();
    println!("Response json: {:#?}", response_json);

    assert!(response_json.is_array(), "Response is not an array");
        // Check if the array is not empty
    let users = response_json.as_array().unwrap();
    println!("Users Array: {:#?}", users);
    
    assert!(!users.is_empty(), "Response array is empty");
        // Check if each user has an "email" field and it's a string
    for user in users {
        assert!(user.is_object(), "User is not an object");
        assert!(user.get("email").is_some(), "User does not have an 'email' field");
        assert!(user.get("created_at").is_some(), "User does not have an 'email' field");
        assert!(user.get("first_name").is_some(), "User does not have an 'email' field");
        assert!(user.get("last_name").is_some(), "User does not have an 'email' field");
        assert!(user.get("username").is_some(), "User does not have an 'email' field");
        assert!(user.get("is_superuser").is_some(), "User does not have an 'email' field");
        assert!(user.get("is_active").is_some(), "User does not have an 'email' field");
        assert!(user.get("is_staff").is_some(), "User does not have an 'email' field");
        assert!(user.get("last_login").is_some(), "User does not have an 'email' field");
    }

}
#[test]
fn test_create_user() {
    let client = Client::new();

    // Generate a random username using the `fake` crate
    let random_username: String = Username().fake();
    let random_email: String = SafeEmail().fake();
    
    let new_user = json!({
        "username": random_username,
        "email": random_email,
        "password": "securepassword123"
    });

    let response = client.post(format!("{}/auth/users", APP_HOST))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&new_user) 
        .send()
        .unwrap();

    println!("Response: {:?}", response);
   
    assert_eq!(response.status(), StatusCode::CREATED);
    let user: Value = response.json().unwrap();
    println!("Response json: {:#?}", user);
    
    assert_eq!(user, json!({
        "id": user["id"],
        "username": new_user["username"],
        "email": new_user["email"],
        "created_at": user["created_at"],
        "first_name": Value::Null,
        "is_active": true,
        "is_staff": false,
        "is_superuser": false,
        "last_login": Value::Null,
        "last_name": Value::Null,
    }));

    let response = client.delete(format!("{}/auth/users/{}", APP_HOST, user["id"]))
        .send()
        .unwrap();

    println!("Response: {:?}", response);
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
