use url;
use rusqlite;

#[derive(Debug, Fail)]
pub enum Error {
	#[fail(display = "Failed to parse resource URI")]
	UrlParse(#[cause] url::ParseError),

	#[fail(display = "RuSqlite error")]
	RuSqite(#[cause] rusqlite::Error ),
}
//error_chain! {
//    foreign_links {
//        UrlParse(url::ParseError);
//        Rusqlite(rusqlite::Error);
//    }
//}
