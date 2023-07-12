### ZILOG

#### Instalasi
`cargo add --git https://github.com/zahrulsch/zilog.git`

#### atau

```toml
[dependencies]
zilog = { git = "https://github.com/zahrulsch/zilog.git" }
```
<hr/>

#### Penggunaan

```rust
use log::LevelFilter;
use zilog::Zilog;

fn main() {
    let debug_mode = std::env::var("DEBUG")
        .unwrap_or("false".to_string())
        .to_lowercase()
        .parse::<bool>()
        .unwrap_or(false);

    let max_level = match debug_mode {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };

    Zilog::builder()
        .set_app_name("KIOTO:")
        .set_file_log_path("kioto.log")
        .init(max_level)
        .unwrap();

    log::info!("Saya info")
}
```