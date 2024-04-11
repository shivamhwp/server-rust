use std::vec;

use axum::{
    extract::{ Json, Path}, http::{Response, StatusCode}, routing::get, Router
};

use serde::{Deserialize,Serialize};




#[derive(Serialize, Deserialize, Clone)]
struct  Movie {
    id : u32,
    content : String
}

#[derive(Serialize, Deserialize)]
struct  Movies {
    movies  : Vec<Movie>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/movie/:id", get(get_movie))
    .route("/movies", get(get_movies));


    async fn root () -> impl axum::response::IntoResponse {
      let message = "Nothing much here, go to /movies to get a list of movies and /movie/:id to get a specific movie" ;
      Json(message)
        
    }
    async fn get_movies() ->impl axum::response::IntoResponse {
      Json ( Movies{
            movies : vec![
                Movie{
                    id : 1,
                    content : "Fight Club".to_string()
                },
                Movie{
                    id : 2,
                    content : "Memento".to_string()
                },
                Movie{
                    id : 3,
                    content : "Gone Girl".to_string()
                }
            ]  
        })
    }
    async fn get_movie(Path(id): Path<u32>) -> Result<Json<Movie>, Response<String>> {
        let movies = Movies {
            movies: vec![
                Movie {
                    id: 1,
                    content: "Fight Club".to_string(),
                },
                Movie {
                    id: 2,
                    content: "Memento".to_string(),
                },
                Movie {
                    id: 3,
                    content: "Gone Girl".to_string(),
                },
            ],
        };
        match movies.movies.iter().find(|movie| movie.id == id) {
            Some(movie) => Ok(Json(movie.clone())),
            None => Err(Response::builder().status(StatusCode::NOT_FOUND).body("movie not found".to_string()).unwrap()),
        }
    }

    let listener  = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap()
        
}




