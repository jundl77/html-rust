extern crate html_rust;

const TEST: &str = "
fn render() -> DOMTree<String> {
    let code = \"HELLO WORLD!\";

    return html!(
        <div>{ text!(code) }</div>
    );
}
";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile() {
        let result: Option<String> = html_rust::transpile(TEST);

        match result {
            Some(html) => assert_eq!(html, "<div>HELLO WORLD!</div>\n"),
            None       => assert_eq!(0, 1)
        };
    }
}