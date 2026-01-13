Реализирайте сигурно удостоверяване за вашите потребители:

```kotlin
// Създайте данни за потребителя (по възможност на вашия сървър)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Генерирайте SSO token (трябва да се извърши от страна на сървъра!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Добавете в конфигурацията
config.sso = token
```