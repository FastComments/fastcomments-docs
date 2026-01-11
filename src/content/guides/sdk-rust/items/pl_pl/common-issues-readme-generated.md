### 401 — Błędy nieautoryzowane

Jeśli otrzymujesz błędy 401 podczas korzystania z uwierzytelnionego API:

1. **Sprawdź swój klucz API**: Upewnij się, że używasz poprawnego klucza API z panelu FastComments
2. **Zweryfikuj tenant ID**: Upewnij się, że tenant ID odpowiada Twojemu kontu
3. **Format klucza API**: Klucz API powinien być przekazany w Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemy z tokenami SSO

Jeśli tokeny SSO nie działają:

1. **Używaj trybu bezpiecznego w produkcji**: Zawsze używaj `FastCommentsSSO::new_secure()` z Twoim kluczem API w środowisku produkcyjnym
2. **Tylko po stronie serwera**: Generuj tokeny SSO na serwerze, nigdy nie ujawniaj klucza API klientom
3. **Sprawdź dane użytkownika**: Upewnij się, że wszystkie wymagane pola (id, email, username) są dostarczone

### Błędy środowiska uruchomieniowego asynchronicznego

SDK używa tokio do operacji asynchronicznych. Upewnij się, że:

1. Dodaj tokio do swoich zależności:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Użyj środowiska uruchomieniowego tokio:
```rust
#[tokio::main]
async fn main() {
    // Twój kod asynchroniczny tutaj
}
```