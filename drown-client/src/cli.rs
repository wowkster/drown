use clap::{error::ErrorKind, CommandFactory, Parser};
use drown_common::ext::{OptionExt, StrExt};
use url::Url;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 6472;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /* The user can supply either a uri, a host and/or port, or neither. */
    #[arg(value_parser = validate_drown_uri)]
    uri: Option<DrownUri>,
    #[arg(short = 'H', long)]
    host: Option<String>,
    #[arg(short = 'P', long)]
    port: Option<u16>,

    /// The username to use when authenticating with the server.
    #[arg(short = 'u', long)]
    username: Option<String>,
    /// The password to use when authenticating with the server.
    #[arg(short = 'p', long)]
    password: Option<String>,

    /// The database to automatically enter when the connection is established.
    #[arg(short = 'd', long)]
    database: Option<String>,
    /// Print more information about what is happening.
    #[arg(short = 'v', long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
pub struct DrownUri {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

fn validate_drown_uri(s: &str) -> Result<DrownUri, String> {
    let uri = Url::parse(s).map_err(|e| e.to_string())?;

    /* Ensure that parts we never want are not part of the URI */

    if uri.cannot_be_a_base() {
        return Err("URI scheme invalid".to_string());
    }

    if uri.path() != "" {
        return Err("URI path must be empty".to_string());
    }

    uri.query().none_or("URI query must be empty")?;
    uri.fragment().none_or("URI fragment must be empty")?;

    /* Ensure that parts we do want in the URI are valid */

    uri.scheme()
        .matches_or(|s| *s == "drown", "URI scheme must be `drown`".to_string())?;

    let host = uri.host_str().ok_or("URI host is required")?.to_string();
    let port = uri.port().unwrap_or(DEFAULT_PORT);

    let username = uri.username().map_empty_to_none().map(|s| s.to_string());
    let password = uri.password().map(|s| s.to_string());

    Ok(DrownUri {
        host,
        port,
        username,
        password,
    })
}

#[derive(Debug)]
pub struct ClientOptions {
    pub connection: DrownUri,
    pub database: Option<String>,
    pub verbose: bool,
}

pub fn get_client_options_from_args() -> ClientOptions {
    let CliArgs {
        uri,
        host,
        port,
        username,
        password,

        database,
        verbose,
    } = CliArgs::parse();

    let connection = if let Some(uri) = uri {
        // If a URI is provided, the host and port cannot be specified
        if host.is_some() || port.is_some() {
            CliArgs::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    "If a full URI is provided, you cannot also specify `--host` or `--port`",
                )
                .exit();
        }

        // Prefer the arguments over the URI credentials
        DrownUri {
            host: uri.host,
            port: uri.port,
            username: username.or(uri.username),
            password: password.or(uri.password),
        }
    } else {
        DrownUri {
            host: host.unwrap_or(DEFAULT_HOST.to_string()),
            port: port.unwrap_or(DEFAULT_PORT),
            username,
            password,
        }
    };

    ClientOptions {
        connection,
        database,
        verbose,
    }
}
