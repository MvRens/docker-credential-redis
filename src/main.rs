use std::env;
use std::io::Read;

mod store;
use crate::store::redis_store;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        exit_print_usage();
    }

    let action = &args[1];
    let mut input = String::new();

    match std::io::stdin().read_to_string(&mut input)
    {
        Ok(_) =>
        {
            let input = input.trim();

            match action.as_str()
            {
                "store" => store(&input),
                "get" => get(&input),
                "erase" => erase(&input),
                _ => exit_print_usage()
            }
        }

        Err(error) =>
        {
            exit_error(format!("Failed to read STDIN: {}", error));
        }
    }
}


fn store(input: &str)
{
    match json::parse(&input) 
    {
        Ok(parsed) =>
        {
            let server_url = &parsed["ServerURL"];
            let username = &parsed["Username"];
            let secret = &parsed["Secret"];

            if !server_url.is_string() || !username.is_string() || !secret.is_string()
            {
                exit_error("Invalid JSON");
            }

            match redis_store::store(server_url.to_string(), username.to_string(), secret.to_string())
            {
                Ok(_) =>
                {                
                }

                Err(error) =>
                {
                    exit_error(format!("Failed to store entry in Redis: {}", error));
                }
            }
        }

        Err(error) =>
        {
            exit_error(format!("Failed to parse STDIN: {}", error));
        }
    }
}


fn get(server_url: &str)
{
    match redis_store::get(&server_url)
    {
        Ok(result) =>
        {
            match result
            {
                Some(credentials) =>
                {
                    let mut output = json::JsonValue::new_object();
                    output["Username"] = credentials.username.into();
                    output["Secret"] = credentials.secret.into();

                    print!("{}", output.dump());
                }

                None =>
                {
                    exit_error(format!("Server URL not found: {}", server_url));
                }
            }
        }

        Err(error) =>
        {
            exit_error(format!("Failed to query server URL: {}", error));
        }
    }
}


fn erase(server_url: &str)
{
    match redis_store::erase(server_url)
    {
        Ok(_) =>
        {            
        }

        Err(error) =>
        {
            exit_error(format!("Failed to erase server URL: {}", error));
        }
    }
}


fn exit_print_usage()
{
    println!("Usage: docker-credential-redis [store|get|erase]");
    println!(  "See: https://docs.docker.com/engine/reference/commandline/login/#credentials-store");
    std::process::exit(1);
}


fn exit_error<S: AsRef<str> + std::fmt::Display>(error: S)
{
    println!("{}", error);
    std::process::exit(1);
}