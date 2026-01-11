### 401 Errori Non Autorizzati

Se ricevi errori 401 quando usi l'API autenticata:

1. **Controlla la tua API key**: Assicurati di usare la API key corretta dalla dashboard di FastComments
2. **Verifica il tenant ID**: Assicurati che il tenant ID corrisponda al tuo account
3. **Formato della API key**: La API key deve essere passata nella Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemi con i token SSO

Se i token SSO non funzionano:

1. **Usa la modalità sicura in produzione**: Usa sempre `FastCommentsSSO::new_secure()` con la tua API key per la produzione
2. **Solo lato server**: Genera i token SSO sul tuo server, non esporre mai la tua API key ai client
3. **Controlla i dati dell'utente**: Assicurati che tutti i campi obbligatori (id, email, username) siano forniti

### Errori del runtime asincrono

Lo SDK usa tokio per operazioni asincrone. Assicurati di:

1. Aggiungi tokio alle tue dipendenze:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Usa il runtime tokio:
```rust
#[tokio::main]
async fn main() {
    // Il tuo codice asincrono qui
}
```