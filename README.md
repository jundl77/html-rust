# html-rust

A simple, light-weight server that transpiles Rust code to HTML. 

This project arose from the need to build React components using Rust. Under the hood, html-rust uses [typed-html](https://github.com/bodil/typed-html). Look at typed-html to see what Rust code you can write and how it is transformed to HTML.

### Example:

Rust code:

```rust
fn render() -> DOMTree<String> {
    let code = "HELLO WORLD!";

    return html!(
        <div>{ text!(code) }</div>
    );
}
```

Resulting HTML:

```html
<div>HELLO WORLD!</div>
```

## API

### /transpile
Make a POST request to ```/transpile``` with the following payload as JSON:

```json
{
  code: "code to transpile"
}
```

**Important:** The code has to be formatted as shown in the example above, providing ```fn render() -> DOMTree<String> ```.

### /status
You can check the status of the service with a simple GET request to ```/status```. If everything is okay, 200 should be returned.

## Docker 

You can find this server as a docker image under [julianbrendl/html-rust](https://hub.docker.com/r/julianbrendl/html-rust/).