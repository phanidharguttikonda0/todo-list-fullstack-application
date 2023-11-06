


    // here we are going to connect to the database


pub mod data_base_connection {
    use sqlx::{Connection, Pool, Postgres, Row};
    use sqlx::postgres::PgPoolOptions;
    use serde::{Serialize, Deserialize} ;
    use chrono::NaiveDateTime;


    #[derive(Serialize, Deserialize,Debug, sqlx::FromRow)]
    pub struct Task {
        name: String,
        start_time: String,
        end_time: String,
    }

    #[derive()]
   pub struct database {
        connection: Pool<Postgres>, // Connection reference
    }


    impl database{
        pub async fn new() -> database {
            let URL:String = String::from("postgres://postgres:Phani@9090K@localhost:5432/todolist") ;
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&URL).await.expect("Unable to connect") ;
            database{
                connection: pool
            }
        }


        pub async fn post_signup(&mut self, email: String, mobile_number: String, username: String, password: String) -> bool {


            let row1 = sqlx::query("SELECT * FROM user_credentials WHERE username = $1")
                .bind(&username)
                .fetch_optional(&self.connection)
                .await.expect("unable to get the row") ;

            let row2 = sqlx::query("SELECT * FROM user_credentials WHERE email = $1")
                .bind(&email)
                .fetch_optional(&self.connection)
                .await.expect("unable to get the row") ;


            let row3 = sqlx::query("SELECT * FROM user_credentials WHERE mobile = $1")
                .bind(&mobile_number)
                .fetch_optional(&self.connection)
                .await.expect("unable to get the row") ;

            if row1.is_some() || row2.is_some() || row3.is_some(){
                return false ;
            }




            let rows_affected = sqlx::query("INSERT INTO user_credentials (email, mobile, username, password) VALUES ($1, $2, $3, $4)")
                .bind(email)
                .bind(mobile_number)
                .bind(username)
                .bind(password)
                .execute(&self.connection)
                .await.expect("Some thing went wrong") ;

            println!("Inserted {:?} rows into my_table", rows_affected);

            true

        }


        pub async fn check_details(&mut self, username: String, password: String) -> bool {

            let row = sqlx::query("SELECT * FROM user_credentials WHERE username = $1")
                .bind(username)
                .fetch_optional(&self.connection)
                .await.expect("unable to get the row") ;

            if row.is_some(){ // if row is there then we can check for password match
                if (row).unwrap().get::<String, _>("password") == (password)  { true  }
                else { false }
            }else{
                false
            }

        }


        pub async fn post_task_details(&mut self, taskname: String, starttime: String, endtime: String, username: String) -> bool{

            // we are going add signup

            // we will return true if the same two tasks are not at the same time

            // the table-name will be current-tasks-username, completed-tasks-username, uncompleted-tasks-username

            // Select * FROM information_schema.tables WHERE table_name = 'current-tasks-username';


        let table_name = "current-tasks-".to_owned()+ &*username;
        let query = format!("CREATE TABLE {} (
  task_name VARCHAR(255) NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL
)", table_name);

        let table_name1 = "completed-tasks-".to_owned()+ &*username;
        let query1 = format!("CREATE TABLE {} (
  task_name VARCHAR(255) NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL
)", table_name1);

        let table_name2 = "uncompleted-tasks-".to_owned()+ &*username;

        let query2 = format!("CREATE TABLE {} (
  task_name VARCHAR(255) NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL
)", table_name2);

            let result1 = sqlx::query(&query)
                .execute(&self.connection)
                .await.expect("") ;

            let result2 = sqlx::query(&query1)
                .execute(&self.connection)
                .await.expect("") ;

            let result3 = sqlx::query(&query2)
                .execute(&self.connection)
                .await.expect("") ;


            // now we are going to insert the values in to the tables

        let query_insert = format!("INSERT INTO {} (id, name, start_time, end_time) VALUES($1, $2, $3)",&table_name) ;
        sqlx::query(&query_insert)
            .bind(taskname).bind(starttime).bind(endtime)
            .execute(&self.connection)
            .await.expect("Unable to insert") ;

            true
        }




        pub async fn get_completed_tasks(&mut self, username: String) -> Vec<Task> {
            let table_name = "current-tasks-".to_owned()+ &*username;
            let query = format!("SELECT * FROM {} ", table_name);
            let values = sqlx::query_as::<_, Task>(&query)
                .fetch_all(&self.connection)
                .await.expect("Unable to get") ;
            values
        }


        pub async fn get_uncompleted_tasks(&mut self, username: String) -> Vec<Task> {
            let table_name = "current-tasks-".to_owned()+ &*username;
            let query = format!("SELECT * FROM {}", table_name);
            let values = sqlx::query_as::<_, Task>(&query)
                .fetch_all(&self.connection)
                .await.expect("Unable to get") ;
            values
        }

        pub async fn get_current_tasks(&mut self, username: String) -> Vec<Task> {
            let table_name = "current-tasks-".to_owned()+ &*username;
            let query = format!("SELECT * FROM {}", table_name);
            let values = sqlx::query_as::<_, Task>(&query)
                .fetch_all(&self.connection)
                .await.expect("Unable to get") ;
            values
        }



        pub async fn post_uncompleted_tasks(&mut self, taskname: String, starttime: String, endtime: String, username: String) {
            let table_name = "uncompleted-tasks-".to_owned()+ &*username;
            let query_insert = format!("INSERT INTO {} (id, name, start_time, end_time) VALUES($1, $2, $3)",&table_name) ;
            let new_starttime = NaiveDateTime::parse_from_str(&starttime, "%Y-%m-%d %H:%M:%S").unwrap();

            let new_endtime = NaiveDateTime::parse_from_str(&endtime, "%Y-%m-%d %H:%M:%S").unwrap();
            sqlx::query(&query_insert)
                .bind(taskname).bind(new_starttime).bind(new_endtime)
                .execute(&self.connection)
                .await.expect("Unable to insert") ;
        }

        pub async fn post_completed_tasks(&mut self, taskname: String, starttime: String, endtime: String, username: String) {
            let table_name = "completed-tasks-".to_owned()+ &*username;
            let query_insert = format!("INSERT INTO {} (id, name, start_time, end_time) VALUES($1, $2, $3)",&table_name) ;
            let new_starttime = NaiveDateTime::parse_from_str(&starttime, "%Y-%m-%d %H:%M:%S").unwrap();

            let new_endtime = NaiveDateTime::parse_from_str(&endtime, "%Y-%m-%d %H:%M:%S").unwrap();
            sqlx::query(&query_insert)
                .bind(taskname).bind(new_starttime).bind(new_endtime)
                .execute(&self.connection)
                .await.expect("Unable to insert") ;
        }


        pub async fn close_connection(&mut self) {
            self.connection.close().await;
        }


    }


}