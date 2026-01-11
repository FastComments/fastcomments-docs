### 401 Неовлашћене грешке

Ако добијате 401 грешке при коришћењу аутентификованог API-ја:

1. **Провјерите ваш API кључ**: Уверите се да користите исправан API кључ из вашег FastComments контролног панела
2. **Потврдите tenant ID**: Уверите се да tenant ID одговара вашем налогу
3. **Формат API кључа**: API кључ треба да се преда у Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Проблеми са SSO токенима

Ако SSO токени не функционишу:

1. **Користите сигуран режим у продукцији**: Увек користите `FastCommentsSSO::new_secure()` са вашим API кључем за продукцију
2. **Само на серверу**: Генеришите SSO токене на вашем серверу, никада не излажите ваш API кључ клијентима
3. **Провјерите податке корисника**: Уверите се да су сви обавезни подаци (id, email, username) обезбеђени

### Грешке асинхроног runtime-а

SDK користи tokio за асинхроне операције. Обавезно:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Ваш асинхрони код овде
}
```