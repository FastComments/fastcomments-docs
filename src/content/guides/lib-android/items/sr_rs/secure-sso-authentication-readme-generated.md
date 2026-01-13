Омогућите сигурну аутентификацију за ваше кориснике:

```kotlin
// Креирајте податке корисника (најбоље на вашем серверу)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Генеришите SSO токен (то треба урадити на серверу!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Додајте у конфигурацију
config.sso = token
```