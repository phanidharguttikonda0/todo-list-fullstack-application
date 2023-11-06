mod Services;
use Services::data_base_connection ;

// we are going to handle the bussiness logic and the requests in this page


pub mod Handlers {
    use actix_web::{HttpRequest, HttpResponse, Responder, web};
    use sqlx::types::Json;
    use serde::{Serialize, Deserialize} ;
    use crate::Handlers::Services::data_base_connection::database;
    use chrono::NaiveDateTime;

    #[derive(Serialize, Deserialize)]
    pub struct SignUp{
        email: String,
        mobile_number: String,
        username : String,
        password : String,
    }


    #[derive(Serialize, Deserialize)]
    pub struct Task{
        name: String,
        start_time: String,
        end_time: String,
    }

    //* firstly this function is called to create a data-base connection


    // we are going to get all the current-tasks when the application loads
    pub async fn get_handler(req: HttpRequest) -> HttpResponse {

        // creating a connection

        let mut database_connection = database::new().await ;

        //* now we are going to connect to database
        let connect = database::new().await ;
        let username = match req.headers().get("username") {
            Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
            None => String::new(), // Handle the case when the header is not present
        };

        // we will get the current tasks, uncompleted tasks and the completed tasks
        let get_current_tasks = database_connection.get_current_tasks(String::from(&username)).await ;
        let get_uncompleted_tasks = database_connection.get_uncompleted_tasks(String::from(&username)).await ;
        let get_completed_tasks = database_connection.get_completed_tasks(String::from(&username)).await ;

        // closing the connection
        database_connection.close_connection().await;

        let serialized_response = serde_json::json!({
        "current_tasks": get_current_tasks,
        "uncompleted_tasks": get_uncompleted_tasks,
        "completed_tasks": get_completed_tasks,
    });

        (HttpResponse::Ok().json(serialized_response))
    }

    pub async fn sign_in(req: HttpRequest) -> impl Responder {

        // creating a connection

        let mut database_connection = database::new().await ;

        let username = match req.headers().get("username") {
            Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
            None => String::new(), // Handle the case when the header is not present
        };

        let password =match req.headers().get("password") {
            Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
            None => String::new(), // Handle the case when the header is not present
        };
        // now we are going to use the username and the password to check
        let boolean = database_connection.check_details(String::from(&username), String::from(&password)).await ;

        // closing the connection
        database_connection.close_connection().await;

        if boolean {
            HttpResponse::Ok().body("true")
        }else{
            HttpResponse::Ok().body("false")
        }

    }

    pub async fn sign_up(req: HttpRequest, data: web::Json<SignUp>) -> std::io::Result<HttpResponse>{

        // creating a connection

        let mut database_connection = database::new().await ;

        // it's a post request
        let username = match req.headers().get("username") {
            Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
            None => String::new(), // Handle the case when the header is not present
        };


        database_connection.post_signup(String::from(&data.email),
        String::from(&data.mobile_number), String::from(&data.username),
            String::from(&data.password)
        ).await ;


        // closing the connection
        database_connection.close_connection().await ;

       Ok(HttpResponse::Ok().finish())

    }

    pub async fn task_creation(req: HttpRequest, data: web::Json<Task>) -> std::io::Result<HttpResponse>{

        // creating a connection

        let mut database_connection = database::new().await ;

        // it's a post request
        let username = match req.headers().get("username") {
            Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
            None => String::new(), // Handle the case when the header is not present
        };


        database_connection.post_task_details(
            String::from(&data.name), String::from(&data.start_time),
            String::from(&data.end_time), String::from(username)
        ).await ;

        // closing the connection
        database_connection.close_connection().await ;

        Ok(HttpResponse::Ok().finish())
    }


}