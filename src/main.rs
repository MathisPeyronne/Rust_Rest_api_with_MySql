#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive; //
#[macro_use]
extern crate rocket_contrib; // maintain cors (attach)
#[macro_use]
extern crate lazy_static; // store coming data

#[macro_use]
extern crate rocket_cors; // originable

use mysql::prelude::*;
use mysql::*;
use rocket::request::Form;
use rocket::response::content::Html;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;

use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue}; // store in json format
use std::collections::HashMap; // store data in hashmap
use std::sync::{Arc, Mutex}; // capture data coming through lazy statics // Html Attributes (Http methods get, post)

// two different platforms
use rocket_cors::{
    AllowedHeaders, // wo kiya data le k aa raha hai kahan say aa rahi hai,
    AllowedOrigins,
    Cors,
    CorsOptions, // headers tells from where the request came
    Error,
};

use rocket::State; // tells about server condition

type ID = usize; // declaring globally
#[derive(Debug, PartialEq, Eq, Deserialize)] // data in the form of bytes, deserialization
struct Message {
    id: ID,
    contents: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Student {
    sid: ID,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct film {
    title: Option<String>,
    note: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Movies_liked_or_recommended {
    student: Student,
    movies: Vec<String>,
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:5500/class_06/index.html",
        "http://rust-rest-api.surge.sh/",
        // allow request from these
        // allow from local machine
    ]);
    CorsOptions {
        allowed_origins: AllowedOrigins::All, //changed it
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true, // without user name and password
        ..Default::default()
    }
    .to_cors() // convert to cross origin
    .expect("Error while building the Cros")
}

//------------------------------get Request to send data in json-------------------
#[get("/")]
fn getRequest() -> JsonValue {
    let mut data = fetch();

    data
}

//------------------------------put request to update data-------------------------
#[put("/update", data = "<user_input>")]
fn edit(user_input: Json<Student>, map: State<'_, MessageMap>) -> JsonValue {
    let res: Student = user_input.into_inner();
    update(res);
    json!({"status":"okay"})
}

//------------------------------delete request to delete data---------------------
#[delete("/delete/<id>")]
fn deleted(id: i32) {
    delete(id);
}

//-----------------------------post request to store data-----------------------
// Mutex for real time store data on server.
type MessageMap = Mutex<HashMap<ID, Option<String>>>;
#[post("/add", data = "<user_input>")]
fn helloPost(user_input: Json<Student>, map: State<'_, MessageMap>) -> JsonValue {
    println!("{:?}", user_input.0.name);
    println!("{:?}", user_input.0.email);
    println!("{:?}", user_input.0.age);

    let res: Student = user_input.into_inner();
    let result = insert(res);

    result
}

//------------------------------get recommendations data-------------------------
#[put("/get_recommendations", data = "<user_input>")]
fn get_recommendation(
    user_input: Json<Movies_liked_or_recommended>,
    map: State<'_, MessageMap>,
) -> JsonValue {
    let res: Movies_liked_or_recommended = user_input.into_inner();
    let mut data = get_recommendations(res);
    //json!({"status":"okay"})

    data
}

// ---------------------------main function for rocket launch------------------------

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![getRequest, helloPost, edit, deleted, get_recommendation],
        )
        .attach(make_cors())
        .manage(Mutex::new(HashMap::<ID, Option<String>>::new()))
}

fn main() {
    rocket().launch();
}

//------------------------------Insert DAta into database-------------------------
fn insert(student: Student) -> JsonValue {
    let pool = Pool::new("mysql://root:root@localhost:3306/film").unwrap();

    let mut conn = pool.get_conn().unwrap();
    let students = vec![student];

    let b = conn
        .exec_batch(
            r"INSERT INTO student (name, email, age)
          VALUES (:name, :email, :age)",
            students.iter().map(|p| {
                params! {
                    "name" => &p.name,
                    "email" => &p.email,
                    "age"=>&p.age
                }
            }),
        )
        .unwrap();

    let c = conn.last_insert_id();
    println!("c value is : {:?}", c);
    json!({ "id": c })
}

//---------------------------------get data from database----------------------
fn fetch() -> JsonValue {
    let pool = Pool::new("mysql://root:root@localhost:3306/film").unwrap();

    let mut conn = pool.get_conn().unwrap();
    let selected_payments = conn
        .query_map(
            "SELECT sid, name, email, age from student",
            |(sid, name, email, age)| Student {
                sid,
                name,
                email,
                age,
            },
        )
        .unwrap();

    json!(selected_payments)
}

//--------------------------------update data in database----------------------
fn update(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/film").unwrap();
    let mut conn = pool.get_conn().unwrap();

    let students = vec![student];

    conn.exec_batch(
        r"UPDATE student 
        set
        name=:name,
        email=:email,
        age=:age 
        where sid=:sid",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
                "name" => &p.name,
                "email" => &p.email,
                "age"=>&p.age
            }
        }),
    )
    .unwrap();

    println!("updated successfully");
}

//--------------------------------delete data from database----------------------
fn delete(id1: i32) {
    let pool = Pool::new("mysql://root:root@localhost:3306/film").unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.exec_drop(
        r"delete from student 
        where sid=:sid",
        params! {
            "sid"=> id1,
        },
    )
    .unwrap();
    println!("deleted successfully {:?}", id1);
}

//mysql://root:root@localhost:3306/Rust_testing

//**************** What i added *********************/
//--------------------------------custome SQL query to get recommendations ----------------------
// input: movies selected
// process:
//      - take the list of films and build a huge query that gets the right movie recommendations.
// output: movies recommended
fn get_recommendations(movies_liked: Movies_liked_or_recommended) -> JsonValue {
    let pool = Pool::new("mysql://root:root@localhost:3306/film").unwrap();
    let mut conn = pool.get_conn().unwrap();

    let students = vec![movies_liked.student];
    let liked_films: Vec<String> = movies_liked.movies;
    println!("Likes movies: {:?}", liked_films);

    // my SQL request
    /*
    let recommended_movies2 = conn
        .query_map(
            build_sql_recommendation_query(liked_films),
            |row: mysql::Row| -> String { row.get(0).unwrap() },
        )
        .unwrap();
    */
    //just update/create the views, don't care about the result. should probably use exec_drop

    let _ = conn
        .query_map(build_sql_query_views(&liked_films), |(title, note)| film {
            title,
            note,
        })
        .unwrap();

    let recommended_movies2 = conn
        .query_map(
            build_sql_recommendation_query(&liked_films),
            |(title, note)| film { title, note },
        )
        .unwrap();
    println!("recommended movies: {:?}", recommended_movies2);

    println!("finished the request");

    //println!("recommended movies: {:#?}", recommended_movies2);

    //***************************************************************/
    // conn.exec_batch(
    //     r"UPDATE student
    //     set
    //     name=:name,
    //     email=:email,
    //     age=:age
    //     where sid=:sid",
    //     students.iter().map(|p| {
    //         params! {
    //             "sid" => p.sid,
    //             "name" => &p.name,
    //             "email" => &p.email,
    //             "age"=>&p.age
    //         }
    //     }),
    // )
    // .unwrap();

    // let recommended_movies = conn
    //     .query_map(
    //         "SELECT sid, name, email, age from student",
    //         |(sid, name, email, age)| Student {
    //             sid,
    //             name,
    //             email,
    //             age,
    //         },
    //     )
    //     .unwrap();
    // println!("{:?}", recommended_movies);
    // json!(selected_payments);

    println!("updated successfully");

    return json!(recommended_movies2);
}

fn build_sql_query_views(movies_liked: &Vec<String>) -> String {
    let mut query = String::from(
        "/* Création de tables qui contiennent uniquement les films cochés */
    CREATE OR REPLACE VIEW films_fav AS
    SELECT film_id,titre,annee,note,duree
    FROM films
    WHERE ",
    );
    for (i, movie) in movies_liked.iter().enumerate() {
        query.push_str(&format!("titre='{}'", movie));
        if i != movies_liked.len() - 1 {
            query.push_str(" OR ");
        }
    }
    query.push_str(";
    
    /*Compter quels sont les genres favoris*/
    CREATE OR REPLACE VIEW genres_fav AS
    SELECT film_genres.film_id, genre_id
    FROM film_genres,films_fav
    WHERE film_genres.film_id=films_fav.film_id;
    
    /* De même pour les realisateurs */
    CREATE OR REPLACE VIEW reals_fav AS
    SELECT film_realisateurs.film_id, realisateur_id
    FROM film_realisateurs,
         films_fav
    WHERE film_realisateurs.film_id = films_fav.film_id;
    
    
    /* Les acteurs */
    CREATE OR REPLACE VIEW acteurs_fav AS
    SELECT film_acteurs.film_id,acteurs.cast_actor_id,nom
    FROM acteurs,films_fav,film_acteurs
    WHERE film_acteurs.film_id=films_fav.film_id AND acteurs.cast_actor_id=film_acteurs.cast_actor_id;");

    let Query = "/* Création de tables qui contiennent uniquement les films cochés */
    CREATE OR REPLACE VIEW films_fav AS
    SELECT film_id,titre,annee,note,duree
    FROM films
    WHERE titre='12 Angry Men' OR titre='Vertigo' OR titre='Spartacus' OR titre='West Side Story' OR titre='The Man in Grey' OR titre='Top Hat';

    /*Compter quels sont les genres favoris*/
    CREATE OR REPLACE VIEW genres_fav AS
    SELECT film_genres.film_id, genre_id
    FROM film_genres,films_fav
    WHERE film_genres.film_id=films_fav.film_id;

    /* De même pour les realisateurs */
    CREATE OR REPLACE VIEW reals_fav AS
    SELECT film_realisateurs.film_id, realisateur_id
    FROM film_realisateurs,
         films_fav
    WHERE film_realisateurs.film_id = films_fav.film_id;


    /* Les acteurs */
    CREATE OR REPLACE VIEW acteurs_fav AS
    SELECT film_acteurs.film_id,acteurs.cast_actor_id,nom
    FROM acteurs,films_fav,film_acteurs
    WHERE film_acteurs.film_id=films_fav.film_id AND acteurs.cast_actor_id=film_acteurs.cast_actor_id;
    ".to_string();

    println!("{}", query);
    query
}

fn build_sql_recommendation_query(movies_liked: &Vec<String>) -> String {
    //"SELECT titre FROM films".to_string()

    //fist make it with films specified below
    let Query = "
    SELECT DISTINCT films.titre, films.note/* , films.note, films.annee, films.duree le DISTINCT aussi */
    FROM films
    JOIN film_genres ON films.film_id = film_genres.film_id
    JOIN film_realisateurs ON films.film_id = film_realisateurs.film_id
    JOIN film_acteurs ON films.film_id = film_acteurs.film_id
    JOIN acteurs a ON film_acteurs.cast_actor_id = a.cast_actor_id
    WHERE film_genres.genre_id = (
        SELECT genre_id
        FROM (
            SELECT genre_id, COUNT(*) as frequency
            FROM genres_fav
            GROUP BY genre_id
            ORDER BY frequency DESC
            LIMIT 1
        ) AS most_frequent_genre
    ) OR film_realisateurs.realisateur_id = (
        SELECT realisateur_id
        FROM (
            SELECT realisateur_id, COUNT(*) as frequency
            FROM reals_fav
            GROUP BY realisateur_id
            ORDER BY frequency DESC
            LIMIT 1
        ) AS most_frequent_real
    ) OR a.cast_actor_id = (
        SELECT cast_actor_id
        FROM (
            SELECT cast_actor_id, COUNT(*) as frequency
            FROM acteurs_fav
            GROUP BY cast_actor_id
            ORDER BY frequency DESC
            LIMIT 1
        ) AS most_frequent_actor
    )
    ORDER BY note DESC
    LIMIT 10;".to_string();
    Query
}

/*

/*
 Imaginons l'utilisateur a choisi: "12 Angry Men", "Vertigo", "Spartacus", "West Side Story", "The Man in Grey", "Top Hat"
 */

/* Création de tables qui contiennent uniquement les films cochés */
CREATE OR REPLACE VIEW films_fav AS
SELECT film_id,titre,annee,note,duree
FROM films
WHERE titre="12 Angry Men" OR titre="Vertigo" OR titre="Spartacus" OR titre="West Side Story" OR titre="The Man in Grey" OR titre="Top Hat";

/*Compter quels sont les genres favoris*/
CREATE OR REPLACE VIEW genres_fav AS
SELECT film_genres.film_id, genre_id
FROM film_genres,films_fav
WHERE film_genres.film_id=films_fav.film_id;

/* De même pour les realisateurs */
CREATE OR REPLACE VIEW reals_fav AS
SELECT film_realisateurs.film_id, realisateur_id
FROM film_realisateurs,
     films_fav
WHERE film_realisateurs.film_id = films_fav.film_id;


/* Les acteurs */
CREATE OR REPLACE VIEW acteurs_fav AS
SELECT film_acteurs.film_id,acteurs.cast_actor_id,nom
FROM acteurs,films_fav,film_acteurs
WHERE film_acteurs.film_id=films_fav.film_id AND acteurs.cast_actor_id=film_acteurs.cast_actor_id;


/* Proposer les meilleur film de chaque genre, acteur et réalisateur aimé.  */
SELECT DISTINCT films.titre, films.note, films.annee, films.duree
FROM films
JOIN film_genres ON films.film_id = film_genres.film_id
JOIN film_realisateurs ON films.film_id = film_realisateurs.film_id
JOIN film_acteurs ON films.film_id = film_acteurs.film_id
JOIN acteurs a ON film_acteurs.cast_actor_id = a.cast_actor_id
WHERE film_genres.genre_id = (
    SELECT genre_id
    FROM (
        SELECT genre_id, COUNT(*) as frequency
        FROM genres_fav
        GROUP BY genre_id
        ORDER BY frequency DESC
        LIMIT 1
    ) AS most_frequent_genre
) OR film_realisateurs.realisateur_id = (
    SELECT realisateur_id
    FROM (
        SELECT realisateur_id, COUNT(*) as frequency
        FROM reals_fav
        GROUP BY realisateur_id
        ORDER BY frequency DESC
        LIMIT 1
    ) AS most_frequent_real
) OR a.cast_actor_id = (
    SELECT cast_actor_id
    FROM (
        SELECT cast_actor_id, COUNT(*) as frequency
        FROM acteurs_fav
        GROUP BY cast_actor_id
                ORDER BY frequency DESC
        LIMIT 1
    ) AS most_frequent_actor
)
ORDER BY note DESC;

*/
