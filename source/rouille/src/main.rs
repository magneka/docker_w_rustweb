#[allow(unused_imports)]
use rouille::{self, Request, Response, post_input, try_or_400};
use serde::{Deserialize, Serialize};

#[derive (Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
struct JamJar {
    flavor: String,
    volume: f32,
}

fn main() {
   

    println!("Starting a jamserver with address http://0.0.0.0:80");

    // LAGT TIL POST AV JSON OG FORM
    rouille::start_server_with_pool("0.0.0.0:80", Some(6),
    | req: &Request | { 
        rouille::router!(req, 
            (GET) (/) => {
                Response::empty_204()
            },
            (GET) (/jam) => {
                let jar = JamJar {
                    flavor: "Strawberry".to_string(),
                    volume: 255.0,
                };
                Response::json(&jar)   
            },
            (POST) (/jam) => {
                let json: JamJar = try_or_400!(rouille::input::json_input(req));
                Response::text(format!("flavor's value is {}", json.flavor))
            },
            (POST) (/form) => {         
                    let input = try_or_400!(post_input!(req, {
                        field1: u32,
                        field2: String,
                    }));
                
                    Response::text(format!("the value of field1 is: {}", input.field1))                
            },
            (GET) (/jamform) => {
                // Show a html page with form
                rouille::Response::html(JAMFORM)
            },
            (POST) (/jamform) => {handle_jamform(req)},
            _ => {
                Response::empty_404()
            }
        )         
    });            
}

fn handle_jamform(request: &Request) -> Response {
    let input = try_or_400!(post_input!(request, {
        jamflavor: String,
        order_count: u32,
    }));

    Response::text(format!("the jamflavor is: {}", input.jamflavor))
}

// HTML document for jamform.
static JAMFORM: &str = r#"
<html>
    <head>
        <title>JAMFORM</title>
    </head>
    <body>
        <form action="/jamform" method="post">
        <p>
            <label for="jamflavor">Flavor:</label>
            <input type="text" id="jamflavor" name="jamflavor" />
        </p>
        <p>
            <label for="order_count">Order count:</label>
            <input type="order_count" id="order_count" name="order_count" />
        </p>   
        <p>
            <button type="submit">Post order</button>
        </p>     
        </form>
    </body>
</html>
"#;
