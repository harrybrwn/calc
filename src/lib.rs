mod ast;
mod lex;
mod parser;

/// Execute the code given as a raw string.
///
/// # Examples
///
/// ```
/// use calc::exec;
///
/// match exec("1 + 1") {
///     Ok(result) => assert_eq!(result, 2.0),
///     Err(msg) => panic!(msg),
/// }
/// ```
pub fn exec(text: &str) -> Result<f64, String> {
    let ast = match parser::parse(text) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    Ok(ast::eval(&ast))
}
