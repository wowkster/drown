use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Select,
    From,
    Where,
    Group,
    By,
    Having,
    Order,
    Asc,
    Dec,
    Offset,
    Limit,
}

pub struct IllegalEnumValueError;

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Select => "SELECT",
            Keyword::From => "FROM",
            Keyword::Where => "WHERE",
            Keyword::Group => "GROUP",
            Keyword::By => "BY",
            Keyword::Having => "HAVING",
            Keyword::Order => "ORDER",
            Keyword::Asc => "ASC",
            Keyword::Dec => "DEC",
            Keyword::Offset => "OFFSET",
            Keyword::Limit => "LIMIT",
        }.to_string()
    }
}

impl FromStr for Keyword {
    type Err = IllegalEnumValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "select" => Keyword::Select,
            "from" => Keyword::From,
            "where" => Keyword::Where,
            "group" => Keyword::Group,
            "by" => Keyword::By,
            "having" => Keyword::Having,
            "order" => Keyword::Order,
            "asc" => Keyword::Asc,
            "dec" => Keyword::Dec,
            "offset" => Keyword::Offset,
            "limit" => Keyword::Limit,
            _ => return Err(IllegalEnumValueError),
        })
    }
}