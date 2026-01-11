### 401 Unauthorized грешки

Ако получавате грешки 401 при използване на автентикирано API:

1. **Проверете вашия API ключ**: Уверете се, че използвате правилния API ключ от таблото за управление на FastComments
2. **Потвърдете tenant ID**: Уверете се, че tenant ID съответства на вашия акаунт
3. **Формат на API ключа**: API ключът трябва да бъде подаден в Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Проблеми със SSO токените

Ако SSO токените не работят:

1. **Използвайте сигурен режим за продукция**: Винаги използвайте `FastCommentsSSO::new_secure()` с вашия API ключ в продукционна среда
2. **Само от страна на сървъра**: Генерирайте SSO токени на вашия сървър, никога не излагайте вашия API ключ на клиентите
3. **Проверете данните на потребителя**: Уверете се, че всички задължителни полета (id, email, username) са предоставени

### Грешки на асинхронния runtime

SDK използва tokio за асинхронни операции. Уверете се, че:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Вашият асинхронен код тук
}
```