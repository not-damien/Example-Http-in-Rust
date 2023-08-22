use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService, payload::Json, Object};


struct Api;


//makes sure that user scheama is always followed
#[derive(Object)]
struct User {
    dame: String,
    email: String
}

#[OpenApi]
impl Api {
    //deafault get example
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }


    //Query<Option<String>> represents the url parameters
    #[oai(path = "/goodbye", method = "get")]
    async fn testboi(&self, fname: Query<Option<String>>,lname: Query<Option<String>>) -> PlainText<String> {
        //println!("{}", lname.as_ref().unwrap());//will get string out of parameter but will also panic if none
        let fullname: (Option<String>, Option<String>) = (fname.0,lname.0);//formats the parameters into a tuple 
        match fullname{//now we can match for both having some 
            (Some(a), Some(b)) =>  PlainText(format!("you have a name it's, {} {}", b,a)),
            _ => PlainText(format!("you have nothing(missing one or more parameters)")),
        }
    }
    #[oai(path = "/jayson", method = "get")]
    async fn ret_json_test(&self, _name: Query<Option<String>>) -> Json<User> {
       //User struct tells compiler to ensure all fields are provided
       Json(User {
            dame: "Some Name".to_string(),
            email: "user@example.com".to_string()
        })
    }
    //acepts json body and returns a json
    #[oai(path = "/postit", method = "post")]
    async fn testpost(&self, end:Json<User>) -> Json<User> {
        println!("wdwd");
        end
    }

    #[oai(path = "/", method = "get")]
    async fn hello_world(&self) -> PlainText<String> {
        PlainText(format!("hello world"))
    }

    
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Damien's Example Api", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();//frontend html that lists information about all of the routes
    let app = Route::new().nest("/api", api_service).nest("/", ui);//nest can be chained to add more endpoints

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
