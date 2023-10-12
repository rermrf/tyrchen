use sqlparser::{parser::Parser, dialect::GenericDialect};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT * FROM tbl";
    
    let ast = Parser::parse_sql(&GenericDialect::default(), sql).unwrap();
    println!("{:#?}", ast);
}