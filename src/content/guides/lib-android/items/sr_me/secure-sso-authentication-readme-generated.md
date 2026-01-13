Имплементирајте безбедну аутентификацију за ваше кориснике:

```kotlin
// Креирајте корисничке податке (по могућности на вашем серверу)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Генеришите SSO токен (то би требало да се ради на серверу!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Додајте у конфигурацију
config.sso = token
```