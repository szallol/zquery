pub type Result<T> = std::result::Result<T, ZqError>;

#[derive(Debug, Fail)]
pub enum ZqError {
	#[fail(display = "Failed to parse resource URI")]
	UrlParse(#[cause] ::url::ParseError),

	#[fail(display = "RuSqlite error")]
	RuSqlite(#[cause] rusqlite::Error ),
}

