use url;
use rusqlite;

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
        Rusqlite(rusqlite::Error);
    }
}
