# Facade

**Facade** is a framework to rapidly add web-UI to any Rust program. It let enrich your console or server app with an awesome UI just in a matter of seconds.

**Facade** fuses web-server and WASM-based web-app into Rust program. No external files. No external dependencies. Just your binary.

To achieve that effect **Facade** uses [Yew](https://github.com/DenisKolodin/yew) framework to build a universal UI and use WebSocket connection to interact with your app that supply live updates to UI that renders them reactively.

## Vuetify relations

We reuse CSS components' styles from Vuetify project: https://github.com/vuetifyjs/vuetify

You can read the license of Vuetify here: https://github.com/vuetifyjs/vuetify/blob/master/LICENSE.md

## Use-cases

Uses-cases of this framework are unlimited and include but are not limited to the following cases
*(checkbox list used to show layouts that implemented to make these cases possible)*:

- [x] Dashboards
- [ ] Admin panels for server
- [ ] Installation wizards
- [ ] Logs explorers
- [ ] Business-intelligence panels
- [ ] Swagger-compatible API playgrounds
- [ ] Blockchain explorers
- [ ] Presentations *(yeah, you can create presentations for Rust conferences with Rust soon)*
- [ ] Polls and quizes
- [ ] Interactive docs

**EXTRA:** Mobile compatibility! It already works, but need flexible layouts.

You know that very hard to keep maintainance of the projects without external supports. [Become a sponsor on Patreon](https://www.patreon.com/deniskolodin) to help us bring these cases faster.

## How to use

### Add dependency

Add a dependency to your `Cargo.toml` file:

```toml
[dependencies]
facade = { git = "https://github.com/DenisKolodin/facade" }
```

We used git here since the framework is very fast changed yet.

### Spawn a server

Spawn a server in your main function and take a control object to declare UI and supply **live** updates to it.

```rust
let mut control = facade::main()?;
```

### Declare UI

Create a scene and put it to `Control` instance:

```rust
let page_one = Page(
    "Page One",
    "Live information",
    Row(vec![
        TitledPanel("Server Status", Dynamic("status")),
    ]),
);
let scene = Dashboard(
    "My Dashboard",
    vec![page_one],
);
control.scene(scene);
```

### Send live data

Now you can use `Control` instance to send live updates to UI using unique IDs you used in UI declaration:

```rust
let mut counter = 0;
loop {
    counter += 1;
    control.assign("status", format!("Counter is {}", counter));
    thread::sleep(Duration::from_millis(300));
}
```

And yeah, you supplied updates too fast in this example, but that's not a problem, because **Facade**
accumulate updates before send them, overwrites changed values and send the latest update only.

Start and enjoy live updates by connecting to: http://localhost:12400 port (by default) with your favorite browser.
If you want to check it with your smartphone than set bind address to `FACADE_ADDRESS=0.0.0.0` and connect
to the same port, but to IP address of your PC/Mac (both to avoid sexism).

You can also check ready to use example [here](https://github.com/DenisKolodin/facade-example).

### Tuning

You can use `FACADE_` prefixed variables to control parameters of the **Facade** in your app. For example,
to change address or port you can use `FACADE_ADDRESS` and `FACADE_PORT` environment variables respectively.
