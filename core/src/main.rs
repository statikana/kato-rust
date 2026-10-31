use std::fs::File;
use std::io::Read;

use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use generated::katovisitor::KatoVisitor;


mod visitor;
mod datatype;
mod ast;
mod ast_debug;
mod util;

fn main() {
    let mut file = &File::open("./input.txt").unwrap();

    let mut buffer = Vec::new();
    let result = file.read_to_end(&mut buffer);
    if let Err(_) = result {
        println!("Could not open file");
    } else {
        println!("Read {} bytes", result.unwrap())
    }
    
    let input_stream = InputStream::new(buffer.as_slice());
    let lexer = generated::katolexer::KatoLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = generated::katoparser::KatoParser::new(token_stream);

    let mut visitor = visitor::ASTVisitor {temp_result: Box::new(ast::Node{ data: ast::Data::Null(), span: ast::Span{ start: 0, end: 1 }})};
    let start = parser.scope().unwrap();
    let tree = visitor.visit_scope(&start);
}