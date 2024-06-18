#![feature(proc_macro_expand)]

use std::str::FromStr;
use proc_macro::*;

const EXPANSION_FAIL_MSG: &str = "The provided macro could not be expanded within this macro. Macros given must expand to a literal (string and number statements), due to this function relying on the expand_expr() method. It is possible that this restriction will be lifted, sometime in the distant future.";
const FAILED_TO_GET_MACRO_GROUP_MSG: &str = "No macro group was found. It is expected that two scopes are created, first for the macro statement, second for the written expansion.";
const FAILED_TO_GET_EXPANSION_GROUP_MSG: &str = "No expansion group was found. It is expected that two scopes are created, first for the macro statement, second for the written expansion.";
//const INCORRECT_MACRO_GROUP_DELIMITER_MSG: &str = &("Macro group incorrect delimiter, parenthesis \\".to_owned() + "\"" + "( ... )" + "\\" + "\" is expected.");
//const INCORRECT_EXPANSION_GROUP_DELIMITER_MSG: &str = &("Expansion group incorrect delimiter, parenthesis \\".to_owned() + "\"" + "( ... )" + "\\" + "\"is expected.");

const DELIMITER: Delimiter = Delimiter::Parenthesis;

// PSEUDOCODE
// Get two groups
// Expand macros within the first group
// Get the expansion as a token stream
// Get the second group as a token stream
// Check if both groups equal eachother once converted to strings
// Create a panic!() if they do not

#[proc_macro]
/// Macros given have to expand to a literal.
/// Expected use-case is having a second macro that does this conversion as a post-processing step, that is then used with this macro for automated testing purposes.
/// 
/// Format:
/// assert_expansion!((macro)(expected_expansion))
pub fn assert_expansion(input: TokenStream) -> TokenStream {
    let mut caravan = input.into_iter();

    let Some(macro_group) = caravan.next() else {
        return compile_error_stream(FAILED_TO_GET_MACRO_GROUP_MSG)
    };
    let Some(expansion_group) = caravan.next() else {
        return compile_error_stream(FAILED_TO_GET_EXPANSION_GROUP_MSG)
    };

    let TokenTree::Group(macro_group) = macro_group else {
        return compile_error_stream(FAILED_TO_GET_MACRO_GROUP_MSG)
    };
    let TokenTree::Group(expansion_group) = expansion_group else {
        return compile_error_stream(FAILED_TO_GET_EXPANSION_GROUP_MSG)
    };

    if macro_group.delimiter() != DELIMITER {
        // INCORRECT_MACRO_GROUP_DELIMITER_MSG
        let incorrect_macro_group_delimiter_msg: &str = &("Macro group incorrect delimiter, parenthesis \\".to_owned() + "\"" + "( ... )" + "\\" + "\" is expected.");

        return compile_error_stream(incorrect_macro_group_delimiter_msg)
    }
    if expansion_group.delimiter() != DELIMITER {
        // INCORRECT_EXPANSION_GROUP_DELIMITER_MSG
        let incorrect_expansion_group_delimiter_msg: &str = &("Expansion group incorrect delimiter, parenthesis \\".to_owned() + "\"" + "( ... )" + "\\" + "\"is expected.");

        return compile_error_stream(incorrect_expansion_group_delimiter_msg)
    }

    let macro_group = macro_group.stream();
    let expansion_group = expansion_group.stream();
    
    let expanded_macro = match macro_group.expand_expr() {
        Ok(ok) => ok,
        Err(err) => { 
            let msg = EXPANSION_FAIL_MSG.to_owned();
            let msg = msg + "\n ExpandError: " + &err.to_string();
            return compile_error_stream(&msg);
        }
    };

    let expansion_group = to_literal_stream(expansion_group);

    let expanded_macro = expanded_macro.to_string();
    let expansion_group = expansion_group.to_string();

    match expanded_macro == expansion_group {
        true => TokenStream::new(),
        false => {
            let Ok(stream) = TokenStream::from_str("panic!()") else {
                return compile_error_stream("Unexpected lex error, when attempting to create a panic!() token stream for an unmatching assertion.")
            };

            return stream
        },
    }
}

fn to_literal_stream(stream: TokenStream) -> TokenStream {
    // To literal
    let literal = stream.to_string();
    let literal = Literal::string(&literal);
    let literal = TokenTree::Literal(literal);

    // To stream
    let mut stream = Vec::new();
    stream.push(literal);
    let stream = stream.into_iter();
    let stream = TokenStream::from_iter(stream);
    
    return stream
}

/// Insertion vulnerable. Input message is flanked by " ", if the input message contains quotes, then it must also contain extra \ to flag those quotes.
/// 
/// Example of a correctly prepared message with quotes: "Macro group incorrect delimiter, parenthesis \\" + "\"" + "( ... )" + "\\" + "\" is expected.";
fn compile_error_stream(msg: &str) -> TokenStream {
    let Ok(stream) = TokenStream::from_str(&("compile_error!(\"".to_owned() + msg + "\")")) else { 
        panic!("Unexpected lex error, when attempting to create a compile_error! token stream.")
    };

    return stream;
}