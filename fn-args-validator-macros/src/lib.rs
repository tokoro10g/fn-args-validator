extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Ident, ItemFn};

#[proc_macro_attribute]
pub fn with_validation(
    attr: proc_macro::TokenStream,
    func: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let vars: Vec<Ident> =
        parse_macro_input!(attr with Punctuated::<Ident, Comma>::parse_terminated)
            .into_iter()
            .collect();
    let func_ast = parse_macro_input!(func as ItemFn);
    with_validation_impl(vars, func_ast).into()
}

fn with_validation_impl(vars: Vec<Ident>, func_ast: ItemFn) -> proc_macro2::TokenStream {
    let mut func_ast = func_ast.clone();
    for var in vars {
        func_ast.block.stmts.insert(
            0,
            parse_quote!(
                {
                    let __result = #var.validate();
                    if __result.is_err() {
                        return Err(__result.err().unwrap());
                    }
                }
            ),
        );
    }
    proc_macro2::TokenStream::from(quote!(#func_ast))
}


#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::quote;
    use syn::{parse2, Ident, ItemFn};
    #[test]
    fn with_validation() {
        let vars: Vec<Ident> = vec![
            Ident::new("x", Span::call_site()),
            Ident::new("y", Span::call_site()),
        ];
        let func: proc_macro2::TokenStream = quote! {
            fn f(x: i32, y: i32) -> i32{
                x + y
            }
        }
        .into();
        let func_ast = parse2::<ItemFn>(func).unwrap();
        let result = super::with_validation_impl(vars, func_ast);
        let expected = "\
TokenStream [
    Ident {
        sym: fn,
    },
    Ident {
        sym: f,
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                sym: x,
            },
            Punct {
                char: ':',
                spacing: Alone,
            },
            Ident {
                sym: i32,
            },
            Punct {
                char: ',',
                spacing: Alone,
            },
            Ident {
                sym: y,
            },
            Punct {
                char: ':',
                spacing: Alone,
            },
            Ident {
                sym: i32,
            },
        ],
    },
    Punct {
        char: '-',
        spacing: Joint,
    },
    Punct {
        char: '>',
        spacing: Alone,
    },
    Ident {
        sym: i32,
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Ident {
                        sym: let,
                    },
                    Ident {
                        sym: __result,
                    },
                    Punct {
                        char: '=',
                        spacing: Alone,
                    },
                    Ident {
                        sym: y,
                    },
                    Punct {
                        char: '.',
                        spacing: Alone,
                    },
                    Ident {
                        sym: validate,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                    Ident {
                        sym: if,
                    },
                    Ident {
                        sym: __result,
                    },
                    Punct {
                        char: '.',
                        spacing: Alone,
                    },
                    Ident {
                        sym: is_err,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [],
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {
                                sym: return,
                            },
                            Ident {
                                sym: Err,
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Ident {
                                        sym: __result,
                                    },
                                    Punct {
                                        char: '.',
                                        spacing: Alone,
                                    },
                                    Ident {
                                        sym: err,
                                    },
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [],
                                    },
                                    Punct {
                                        char: '.',
                                        spacing: Alone,
                                    },
                                    Ident {
                                        sym: unwrap,
                                    },
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [],
                                    },
                                ],
                            },
                            Punct {
                                char: ';',
                                spacing: Alone,
                            },
                        ],
                    },
                ],
            },
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Ident {
                        sym: let,
                    },
                    Ident {
                        sym: __result,
                    },
                    Punct {
                        char: '=',
                        spacing: Alone,
                    },
                    Ident {
                        sym: x,
                    },
                    Punct {
                        char: '.',
                        spacing: Alone,
                    },
                    Ident {
                        sym: validate,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                    Ident {
                        sym: if,
                    },
                    Ident {
                        sym: __result,
                    },
                    Punct {
                        char: '.',
                        spacing: Alone,
                    },
                    Ident {
                        sym: is_err,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [],
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {
                                sym: return,
                            },
                            Ident {
                                sym: Err,
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Ident {
                                        sym: __result,
                                    },
                                    Punct {
                                        char: '.',
                                        spacing: Alone,
                                    },
                                    Ident {
                                        sym: err,
                                    },
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [],
                                    },
                                    Punct {
                                        char: '.',
                                        spacing: Alone,
                                    },
                                    Ident {
                                        sym: unwrap,
                                    },
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [],
                                    },
                                ],
                            },
                            Punct {
                                char: ';',
                                spacing: Alone,
                            },
                        ],
                    },
                ],
            },
            Ident {
                sym: x,
            },
            Punct {
                char: '+',
                spacing: Alone,
            },
            Ident {
                sym: y,
            },
        ],
    },
]\
        ";
        let actual = format!("{:#?}", result);
        assert_eq!(expected, actual);
    }
}
