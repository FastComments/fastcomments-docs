Реализуйте безопасную аутентификацию для ваших пользователей:

```kotlin
// Создайте данные пользователя (в идеале на вашем сервере)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Сгенерируйте SSO-токен (это должно выполняться на сервере!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Добавьте в конфигурацию
config.sso = token
```