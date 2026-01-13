### 401 Neautorizirane pogreške

Ako dobivate 401 pogreške prilikom korištenja autentificiranog API-ja:

1. **Provjerite svoj API key**: Uvjerite se da koristite ispravan API key iz vašeg FastComments dashboarda
2. **Provjerite tenant ID**: Provjerite da tenant ID odgovara vašem računu
3. **Format API ključa**: API key bi trebao biti proslijeđen u Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemi sa SSO tokenima

Ako SSO tokeni ne rade:

1. **Koristite siguran način za produkciju**: Uvijek koristite `FastCommentsSSO::new_secure()` s vašim API key u produkciji
2. **Samo na serverskoj strani**: Generirajte SSO tokene na svom serveru, nikada ne izlažite svoj API key klijentima
3. **Provjerite podatke korisnika**: Osigurajte da su sva obavezna polja (id, email, username) dostavljena

### Pogreške asinkronog runtime-a

SDK koristi tokio za asinkrone operacije. Pobrinite se da:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Koristite tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Ovdje ide vaš asinkroni kod
}
```