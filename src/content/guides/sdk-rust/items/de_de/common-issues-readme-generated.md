### 401 Unauthorized-Fehler

Wenn Sie 401-Fehler erhalten, wenn Sie die authentifizierte API verwenden:

1. **Überprüfen Sie Ihren API-Schlüssel**: Stellen Sie sicher, dass Sie den richtigen API-Schlüssel aus Ihrem FastComments-Dashboard verwenden
2. **Überprüfen Sie die Tenant-ID**: Stellen Sie sicher, dass die Tenant-ID mit Ihrem Konto übereinstimmt
3. **API-Schlüssel-Format**: Der API-Schlüssel sollte in der Configuration übergeben werden:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO-Token-Probleme

Wenn SSO-Tokens nicht funktionieren:

1. **Sicheren Modus in Produktionsumgebungen verwenden**: Verwenden Sie in der Produktion immer `FastCommentsSSO::new_secure()` mit Ihrem API-Schlüssel
2. **Nur serverseitig**: Erzeugen Sie SSO-Tokens auf Ihrem Server, geben Sie Ihren API-Schlüssel niemals an Clients weiter
3. **Überprüfen Sie Benutzerdaten**: Stellen Sie sicher, dass alle erforderlichen Felder (id, email, username) vorhanden sind

### Async-Runtime-Fehler

Das SDK verwendet tokio für asynchrone Operationen. Stellen Sie sicher, dass Sie:

1. Fügen Sie tokio zu Ihren Abhängigkeiten hinzu:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Verwenden Sie die tokio-Runtime:
```rust
#[tokio::main]
async fn main() {
    // Ihr asynchroner Code hier
}
```