Implementirajte sigurnu autentifikaciju za vaše korisnike:

```kotlin
// Kreirajte podatke o korisniku (idealno na vašem serveru)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generišite SSO token (trebalo bi da se radi na serverskoj strani!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Dodajte u konfiguraciju
config.sso = token
```