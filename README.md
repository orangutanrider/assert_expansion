The way this was to be used was for creating tests. You'd create create a version of your main macro that output as a literal, then you could use that to assert the expansion. You don't need to do it this way, because the same thing can be done internally, the following pseudocode illustrates:

```
#[proc_macro]
pub fn main_function_like_macro(input: TokenStream) -> TokenStream {
	Macro stuff
}

#[proc_macro]
#[cfg(all(debug_assertions, not(feature = "no_assertions")))] // It's for tests only
pub fn assertion_of_main_macro(input: TokenStream) -> TokenStream {
	Expect two groups as input
	Use the main macro on the first group (this can be done internally by just using it as a function)
	Take token stream of 2nd group
	Convert expansion of 1st group and token stream of 2nd group to strings
	Compare and create a panic if they do not match
}
```
