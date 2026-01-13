### 401 Неовлашћено

Ако добијате 401 грешке када користите аутентификовани API:

1. **Проверите ваш API кључ**: Уверите се да користите исправан API кључ са вашег FastComments dashboard
2. **Проверите tenant ID**: Уверите се да tenant ID одговара вашем налогу
3. **Формат API кључа**: API кључ треба проследити у Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Проблеми са SSO токенима

Ако SSO токени не раде:

1. **Користите сигуран режим за продукцију**: Увек користите `FastCommentsSSO::new_secure()` са вашим API кључем за продукцију
2. **Само на серверу**: Генеришите SSO токене на свом серверу, никада не излажите свој API кључ клијентима
3. **Проверите податке корисника**: Уверите се да су обезбеђена сва обавезна поља (id, email, username)

### Грешке асинхроног окружења

SDK користи tokio за асинхроне операције. Побрините се да:

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