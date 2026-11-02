use std::fs::File;
use std::io::Read;
use std::time;

use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use generated::katovisitor::KatoVisitorCompat;


mod ast;
mod operator;
mod ast_visitor;
mod typecheck;
mod datatype;
mod ast_debug;

fn read(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();

    let mut buffer = Vec::new();
    let result = file.read_to_end(&mut buffer);
    if let Err(_) = result {
        panic!("Could not open file");
    } else {
        println!("Read {} bytes", result.unwrap())
    }
    return buffer;
}

fn main() {
    let data = read("./input.txt");

    // input.txt ----> token stream -----> syntax tree ---------> AST ----------> Typed AST
    //           lexer              parser             ASTVisitor     typechecker
    println!("Creating input streams...");
    let input_stream = InputStream::new(&data[..]);
    let lexer = generated::katolexer::KatoLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = generated::katoparser::KatoParser::new(token_stream);

    let mut ast_visitor = ast_visitor::ASTVisitor {temp_result: Box::new(ast::ASTNode::new(ast::ASTUnit, ast::Span{ start: 0, end: 1 }))};
    let start = parser.scope().unwrap();

    println!("Start AST...");

    let t_ast_start = time::SystemTime::now();
    let result = ast_visitor.visit_scope(&start);
    let t_ast_end = time::SystemTime::now();
    
    println!("Finished AST");
    println!("Starting typing...");

    let t_typed_start = time::SystemTime::now();
    let typed = typecheck::typecheck(&result).unwrap();
    let t_typed_end = time::SystemTime::now();

    println!("Finished typing");

    // for statement in &typed.children {
    //     println!("{:?}", statement);
    // }

    println!("AST Tree: {:>3}ms", t_ast_end.duration_since(t_ast_start).unwrap().as_millis());
    println!("Typing:   {:>3}ms", t_typed_end.duration_since(t_typed_start).unwrap().as_millis());
    println!("Total:    {:>3}ms", (t_typed_end.duration_since(t_typed_start).unwrap() + t_ast_end.duration_since(t_ast_start).unwrap()).as_millis());
    println!("Exiting...")
}