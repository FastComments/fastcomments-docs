Реалізуйте безпечну аутентифікацію для ваших користувачів:

```kotlin
// Створіть дані користувача (бажано на вашому сервері)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Згенеруйте SSO-токен (повинно виконуватися на сервері!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Додайте до конфігурації
config.sso = token
```