use stackr_rs::*;

fn main() {
    // This is just some ugly code to showcase the formatting.
    // Typically '.' is used as a noop op and helps with formatting.
    let code = r#"
    var 
    stuff 
    .

    1 stuff 
    set .

    stuff
    get 
    .


    : debug "" "" "" print-stack drop ;
    0 begin 1 + dup 2 == if 
            "hello" .  stuff get drop .  break
        end

        dup
        2 == if 
            "hello"
        else
            "world"
            drop
        end
    loop

    "world"
    
    "#;
    let code = Interpreter::<()>::format_code(code, None).unwrap();

    println!("{}", code);
}
