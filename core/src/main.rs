#![feature(generic_const_exprs)]


use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use generated::katovisitor::KatoVisitor;

mod datatype;
mod visitor;
mod ast;
mod std;


fn main() {
    let file = "./input.txt";
    let input_stream = InputStream::new(file);
    let lexer = generated::katolexer::KatoLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = generated::katoparser::KatoParser::new(token_stream);

    let mut visitor = visitor::ASTVisitor {temp_result: Box::new(ast::ASTNode::Null)};
    let start = parser.program().unwrap();
    let tree = visitor.visit_program(&start);



    println!("wow!");
}