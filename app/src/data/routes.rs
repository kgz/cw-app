use std::collections::HashMap;

#[derive(Debug)]
pub struct Route {
    pub description: String,
}

pub fn gen_path(path: &str) -> String {
    format!("/{}", path.replace(" ", "").as_str().trim_end_matches("/")).to_string()
}

pub fn get_routes() -> HashMap<&'static str, Route> {
    let mut hash: HashMap<&str, Route> = HashMap::new();
    hash.insert(
        "bedhead_attractor",
        Route {
            description: "The Bedhead Attractor is a chaotic attractor defined by the following equations: x_{n+1} = sin(x \\cdot y/b) \\cdot y + cos(a \\cdot x - y), y_{n+1} = x + sin(y)/b".to_string(),
        },
    );
    hash.insert(
        "bogdanov_map",
        Route {
            description: "The Bogdanov Map is a chaotic map defined by the following equations: x_{n+1} = y_n + 1 - a \\cdot x_n^2, y_{n+1} = b \\cdot x_n".to_string(),
        },
    );
    hash.insert(
        "brusselator",
        Route {
            description: "The Brusselator is a chaotic map defined by the following equations: x_{n+1} = 1 + x_n + a \\cdot x_n^2 \\cdot y_n - (b + 1) \\cdot x_n, y_{n+1} = b \\cdot x_n - a \\cdot x_n^2 \\cdot y_n".to_string(),
        },
    );
    hash.insert(
        "clifford_attractor",
        Route {
            description: "The Clifford Attractor is a chaotic attractor defined by the following equations: x_{n+1} = sin(a \\cdot y_n) + c \\cdot cos(a \\cdot x_n), y_{n+1} = sin(b \\cdot x_n) + d \\cdot cos(b \\cdot y_n)".to_string(),
        },
    );
    hash.insert(
        "fractal_dream_attractor",
        Route {
            description: "The Fractal Dream Attractor is a chaotic attractor defined by the following equations: x_{n+1} = sin(y_n \\cdot b) + c \\cdot sin(x_n \\cdot b), y_{n+1} = sin(x_n \\cdot a) + d \\cdot sin(y_n \\cdot a)".to_string(),
        },
    );
    
    hash.insert(
        "gumowski-mira_attractor",
        Route {
            description: "The Gumowski-Mira Attractor is a chaotic attractor defined by the following equations: x_{n+1} = b \\cdot y_n + a \\cdot x_n + x_n \\cdot (x_n^2 + y_n^2), y_{n+1} = -b \\cdot x_n + a \\cdot y_n + y_n \\cdot (x_n^2 + y_n^2)".to_string(),
        },
    );
    hash.insert(
        "henon_map",
        Route {
            description: "The Henon Map is a chaotic map defined by the following equations: x_{n+1} = 1 - a \\cdot x_n^2 + y_n, y_{n+1} = b \\cdot x_n".to_string(),
        },
    );
    hash.insert(
        "hopalong_attractor",
        Route {
            description: "The Hopalong Attractor is a chaotic attractor defined by the following equations: x_{n+1} = y_n - sign(x_n) \\cdot \\sqrt{|b \\cdot x_n - c|}, y_{n+1} = a - x_n".to_string(),
        },
    );
    hash.insert(
        "hopalong_attractor_positive",
        Route {
            description: "The Hopalong Attractor Positive is a chaotic attractor defined by the following equations: x_{n+1} = y_n - sign(x_n) \\cdot \\sqrt{|b \\cdot x_n - c|}, y_{n+1} = a - x_n".to_string(),
        },
    );
    hash.insert(
        "hopalong_attractor_additive",
        Route {
            description: "The Hopalong Attractor Additive is a chaotic attractor defined by the following equations: x_{n+1} = y_n - sign(x_n) \\cdot \\sqrt{|b \\cdot x_n - c|}, y_{n+1} = a - x_n".to_string(),
        },
    );
    hash.insert(
        "hopalong_attractorsin",
        Route {
            description: "The Hopalong Attractor (Sin version) is a chaotic attractor defined by the following equations: x_{n+1} = y_n - sign(x_n) \\cdot \\sqrt{|b \\cdot x_n - c|}, y_{n+1} = a - x_n".to_string(),
        },
    );
    hash.insert("gingerbread_man", Route {
        description: "WIP".to_string(),
    });
    hash.insert("ikeda_map", Route {
        description: "The Ikeda Map is a chaotic map defined by the following equations: x_{n+1} = 1 + c \\cdot (x_n \\cdot cos(t) - y_n \\cdot sin(t)), y_{n+1} = c \\cdot (x_n \\cdot sin(t) + y_n \\cdot cos(t))".to_string(),
    });
    hash.insert("mandlebrot_set", Route {
        description: "The Mandlebrot Set is a chaotic map defined by the following equations: x_{n+1} = x_n^2 - y_n^2 + x_0, y_{n+1} = 2 \\cdot x_n \\cdot y_n + y_0".to_string(),
    });



    return hash;
}
