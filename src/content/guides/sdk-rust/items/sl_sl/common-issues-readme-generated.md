### 401 Neavtorizirane napake

Če prejmete 401 napake pri uporabi avtenticiranega API-ja:

1. **Preverite svoj API ključ**: Prepričajte se, da uporabljate pravilen API ključ iz nadzorne plošče FastComments
2. **Preverite tenant ID**: Prepričajte se, da se tenant ID ujema z vašim računom
3. **Oblika API ključa**: API ključ mora biti podan v Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Težave s SSO žetoni

Če SSO žetoni ne delujejo:

1. **Uporabite varen način za produkcijo**: Vedno uporabljajte `FastCommentsSSO::new_secure()` z vašim API ključem za produkcijo
2. **Samo na strežniški strani**: Generirajte SSO žetone na svojem strežniku, nikoli ne razkrivajte svojega API ključa odjemalcem
3. **Preverite podatke o uporabniku**: Prepričajte se, da so vsa zahtevana polja (id, email, username) zagotovljena

### Napake asinhronega izvajalnega okolja

SDK uporablja tokio za asinhrone operacije. Poskrbite za:

1. Dodajte tokio v svoje odvisnosti:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Uporabite tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Vaša asinhrona koda tukaj
}
```