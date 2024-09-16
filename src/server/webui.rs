///! ##################################################
///! ##################################################
///! #
///! # ⚠️ CAUTION: This is an HTTP-only server!
///! #
///! # If you don't know what you're doing, don't run
///! #
///! ##################################################
///! ##################################################

use actix_web::{get, http::header::ContentType, post, web::{self, Form}, App, HttpResponse, HttpServer, Responder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ureq::Response;
use serde_json::json;

use super::*;

/*
from flask import Flask, render_template, request, redirect, url_for, flash, session
import requests
import json
from flask import send_from_directory
import os

app = Flask(__name__)
app.secret_key = os.getenv("FLASK_SECRET_KEY")
*/

// Somehow have an `app` with an `app.secret_key`

// Define a handler that captures path parameters
#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the Actix Web app
    let app = App::new()
        .service(index)  // Register the index route
        .service(favicon)  // Register the favicon route
        .service(greet); // Register the greet route

    let host = "127.0.0.1";
    let port = 13338;

    // Start the server
    HttpServer::new(|| app.clone())
        .bind((host, port))?
        .run()
        .await
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_main() -> Result<()> {
//         main().await?;
//         Ok(())
//     }
// }



fn post<T, R>(endpoint: &str, data: T) -> Result<R>
where
    T: serde::Serializer,
    R: serde::Deserialize,
{
    use ureq;
    use serde_json::json;

    // Create an instance of your data
    // let data = MyData {
    //     name: String::from("John Doe"),
    //     age: 30,
    // };

    // Send a POST request with the JSON data
    let response = ureq::post(endpoint)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!(data))?;

    // Check the response
    // println!("Response: {}", response.into_string()?);
    let r = response.try_into()?;
    Ok(r)
}

/// Send a request to the specified endpoint of an HTTP-only server.
/// 
/// Args:
/// prompt (str): The input prompt for the request.
/// endpoint (str): The endpoint to which the request will be sent.
/// 
/// Returns:
/// str: The response from the server.
/// 
/// Raises:
/// KeyError: If the response JSON does not contain the expected "response" key.
fn send_request<T>(prompt: &str, endpoint: &str, get_bearer: fn()->String) -> Option<String> {
    let base_url = "http://127.0.0.1:13337";
    let url = format!("{base_url}{endpoint}");
    let bearer_token = get_bearer();
    let headers = vec![
        ("Content-Type", "application/json"),
        ("Authorization", format!("Bearer {}", bearer_token)).into()
    ];
    let data = json!({
        "input": prompt
    });
    // Send a POST request with the JSON data
    let response = ureq::post(url.as_str())
        .set("Content-Type", "application/json")
        .send_json(data)
        .ok()?;

    // Check the response
    let response = response.into_string()?;
    println!("Response: {}", response.as_str());
    // let response = response.into_json()?;

    Some(response)
/*
def send_request(prompt, endpoint):
    """    Send a request to the specified endpoint of an HTTP-only server.

    Args:
        prompt (str): The input prompt for the request.
        endpoint (str): The endpoint to which the request will be sent.

    Returns:
        str: The response from the server.

    Raises:
        KeyError: If the response JSON does not contain the expected "response" key.
    """

    base_url = "http://127.0.0.1:13337"
    url = f"{base_url}{endpoint}"
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {session['token']}",
    }
    data = json.dumps({"input": prompt})
    response = requests.post(url, headers=headers, data=data, verify=False)

    try:
        response = requests.post(url, headers=headers, data=data)
        response.raise_for_status()  # raises HTTPError if the response status isn't 200
    except requests.ConnectionError:
        return "Error: Unable to connect to the server."
    except requests.HTTPError as e:
        return f"Error: An HTTP error occurred: {str(e)}"

*/

}

#[get("/favicon.ico")]
fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
        .body(include_bytes!("../../installer/server/webui/favicon.ico").into_iter().collect())
/*
@app.route("/favicon.ico")
def favicon():
    """    Send the favicon.ico file from the static directory.

    Returns:
        Response object with the favicon.ico file

    Raises:
         -
    """

    return send_from_directory(
        os.path.join(app.root_path, "static"),
        "favicon.ico",
        mimetype="image/vnd.microsoft.icon",
    )
*/        
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    prompt: String,
    api: String,
}

fn get_bearer() -> String {
    todo!();
}

// Define a simple handler function
#[get("/")]
#[post("/")]
async fn index(web::Form(form): web::Form<Info>) -> Result<String> {

    let response = if form.prompt.is_empty() {
        None
    } else {
        Some(send_request(&form.prompt, &form.api, get_bearer)?)
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(render_template("index.html", response.as_deref()))
    
/*
    @app.route("/", methods=["GET", "POST"])
    def index():
        """    Process the POST request and send a request to the specified API endpoint.
        Returns:
            str: The rendered HTML template with the response data.
        """
        if request.method == "POST":
            prompt = request.form.get("prompt")
            endpoint = request.form.get("api")
            response = send_request(prompt=prompt, endpoint=endpoint)
            return render_template("index.html", response=response)
        return render_template("index.html", response=None)
*/
}

// Render the HTML template (you'll need to implement this function)
fn render_template(template_name: &str, response: Option<&str>) -> String {
    match template_name {
        "index.html" => {
            
            // load index.html from ../../../installer/server/webui/templates/index.html
            // Load the HTML template at compile time
            const INDEX_TEMPLATE: &str = include_str!("../../installer/server/webui/templates/index.html");

            INDEX_TEMPLATE.to_owned()
        }
        _ => {
            todo!();
            // Implement your template rendering logic here
            format!("<html><body><h1>{}</h1></body></html>", response.unwrap_or(""))
        }
    }
}
