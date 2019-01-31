#[macro_use]
extern crate rouille;
use rouille::Response;

struct Command {
    name: String,
    alias: String,
    args: String,
    dest: String,
}

fn _bisect(parts: Vec<&str>, sep: &str) -> (String, String) {
    let head = parts[0].to_string();
    let tail_ = parts[1..].join(sep);
    let tail = tail_.trim().to_string();
    (head, tail)
}

fn bisect_ws(string: &str) -> (String, String) {
    _bisect(string.split_whitespace().collect(), " ")
}

fn bisect_on(string: &str, sep: &str) -> (String, String) {
    _bisect(string.split(sep).collect(), sep)
}

fn parse_command(line: &str) -> Command {
    let (left, dest)  = bisect_on(&line, "=");
    let (cmd,  args)  = bisect_ws(&left);
    let (name, alias) = bisect_on(&cmd, "|");

    Command { name, alias, args, dest }
}

fn load_commands() -> Vec<Command> {
    include_str!("commands.txt")
        .split_terminator("\n")
        .map(parse_command)
        .collect()
}

fn url_for(commands: &Vec<Command>, cmd: &str, args: &str) -> String {
    for command in commands {
        if command.name == cmd || command.alias == cmd {
            let mut url = command.dest.clone();
            let cmd_args: Vec<_> = command.args.split_whitespace().collect();
            let argvec: Vec<_> = args.split_whitespace().collect();
            for (idx, arg) in command.args.split_whitespace().enumerate() {
                if idx == (cmd_args.len() - 1) {
                    url = url.replace(arg, &argvec[idx..].join(" "));
                } else {
                    url = url.replace(arg, argvec[idx]);
                }
            }
            return format!("{}", url);
        }
    }
    format!("/?invalid=true&q={}", cmd)
}

fn search(commands: &Vec<Command>, request: &rouille::Request) -> rouille::Response {
    if let Some(q) = request.get_param("q") {
        let parts: Vec<_> = q.split_whitespace().collect();
        let cmd = parts[0];
        let args = parts[1..].join(" ");

        Response::redirect_303(url_for(&commands, &cmd, &args))
    } else {
        Response::text("No query provided.")
    }
}

fn home(commands: &Vec<Command>, request: &rouille::Request) -> rouille::Response {
    let q = request.get_param("q").unwrap_or("".to_string());
    let invalid_str = request.get_param("invalid").unwrap_or("".to_string());
    let invalid = invalid_str == "true";

    let mut response = HEAD.to_string();
    if invalid {
        response.push_str(&format!("<p>No such command: {}</p>", q));
    }

    response.push_str(FORM);

    response.push_str("\n\n<p>Commands:</p><ul>\n");
    for cmd in commands {
        response.push_str(&format!("    <li>{} {}</li>\n", cmd.name, cmd.args));
    }
    response.push_str("</ul>\n");

    Response::html(response)
}

fn main() {
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let host = format!("0.0.0.0:{}", port);

    let commands = load_commands();
    for cmd in load_commands() {
        println!("{}({}) = {}", cmd.name, cmd.args, cmd.dest);
        println!("  aka {}", cmd.alias);
    }

    println!("Listening on {}.", host);
    rouille::start_server(host, move |request| {
        router!(request,
                (GET) (/)       => { home(&commands, request) },
                (GET) (/search) => { search(&commands, request) },
                _ => Response::empty_404()
        )
    })
}

static HEAD: &'static str = "<!doctype html>\n";
static FORM: &'static str = r#"
<form action="/search">
    <input type="text" name="q" id="q">
    <input type="submit" value="Go">
</form>
"#;
