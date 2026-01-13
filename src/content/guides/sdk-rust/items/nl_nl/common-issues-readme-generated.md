### 401 Ongeautoriseerde fouten

Als u 401-fouten krijgt bij het gebruik van de geauthenticeerde API:

1. **Controleer uw API key**: Zorg dat u de juiste API key uit uw FastComments-dashboard gebruikt
2. **Verifieer de tenant ID**: Zorg dat de tenant ID overeenkomt met uw account
3. **API key-formaat**: De API key moet worden doorgegeven in de Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemen met SSO-tokens

Als SSO-tokens niet werken:

1. **Gebruik veilige modus voor productie**: Gebruik altijd `FastCommentsSSO::new_secure()` met uw API key in productie
2. **Alleen server-side**: Genereer SSO-tokens op uw server, stel uw API key nooit bloot aan clients
3. **Controleer gebruikersgegevens**: Zorg dat alle vereiste velden (id, email, username) zijn opgegeven

### Fouten van de async-runtime

De SDK gebruikt tokio voor async-bewerkingen. Zorg ervoor dat u:

1. Voeg tokio toe aan uw afhankelijkheden:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Gebruik de tokio-runtime:
```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```