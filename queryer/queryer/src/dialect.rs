use sqlparser::dialect::Dialect;


#[derive(Debug, Default)]
pub struct TryDialect;

// 创建自己的 sql 方言。 TryDialect 支持 identifier 可以是简单的 url
impl Dialect for TryDialect{
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    // indenifier 可以有 ':', '/', '?', '&', '=' (主要目的让sql支持url)
    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

/// 测试辅助函数
pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );
    sql
}

#[cfg(test)]
mod tests {
    use sqlparser::parser::Parser;

    use super::*;

    #[test]
    fn it_works() {
        let p = Parser::parse_sql(&TryDialect::default(), &example_sql()).unwrap();
        println!("{:?}", p);
    }
}
