### 401 Neautorizovane greške

Ako dobijate 401 greške pri korišćenju autentifikovanog API-ja:

1. **Proverite svoj API ključ**: Uverite se da koristite ispravan API ključ sa vaše FastComments kontrolne table
2. **Proverite tenant ID**: Uverite se da tenant ID odgovara vašem nalogu
3. **Format API ključa**: API ključ treba biti prosleđen u Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemi sa SSO tokenima

Ako SSO tokeni ne funkcionišu:

1. **Koristite sigurni režim za produkciju**: Uvek koristite `FastCommentsSSO::new_secure()` sa vašim API ključem za produkciju
2. **Samo na serverskoj strani**: Generišite SSO tokene na svom serveru, nikada ne izlažite svoj API ključ klijentima
3. **Proverite korisničke podatke**: Uverite se da su svi obavezni podaci (id, email, username) obezbeđeni

### Greške asinhronog runtime-a

SDK koristi tokio za asinhrone operacije. Uverite se da:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Vaš asinhroni kod ovde
}
```