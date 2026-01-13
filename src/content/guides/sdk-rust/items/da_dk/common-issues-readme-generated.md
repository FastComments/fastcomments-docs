### 401 Uautoriserede fejl

Hvis du får 401-fejl, når du bruger den autentificerede API:

1. **Kontroller din API-nøgle**: Sørg for, at du bruger den korrekte API-nøgle fra dit FastComments-dashboard
2. **Bekræft tenant-ID'et**: Sørg for, at tenant-ID'et matcher din konto
3. **API-nøglens format**: API-nøglen skal angives i Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO-tokenproblemer

Hvis SSO-tokens ikke virker:

1. **Brug sikker tilstand i produktion**: Brug altid `FastCommentsSSO::new_secure()` med din API-nøgle i produktion
2. **Kun server-side**: Generér SSO-tokens på din server, eksponer aldrig din API-nøgle for klienter
3. **Kontroller brugerdata**: Sørg for, at alle påkrævede felter (id, email, username) er angivet

### Asynkrone runtime-fejl

SDK'en bruger tokio til asynkrone operationer. Sørg for at:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Din asynkrone kode her
}
```