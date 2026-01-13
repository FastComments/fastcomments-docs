Zaimplementuj bezpieczne uwierzytelnianie dla swoich użytkowników:

```kotlin
// Utwórz dane użytkownika (najlepiej na Twoim serwerze)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Wygeneruj token SSO (powinno być wykonane po stronie serwera!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Dodaj do konfiguracji
config.sso = token
```