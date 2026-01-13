Implementirajte sigurnu autentifikaciju za svoje korisnike:

```kotlin
// Kreirajte korisničke podatke (idealno na vašem poslužitelju)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generirajte SSO token (to bi trebalo napraviti na strani poslužitelja!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Dodajte u konfiguraciju
config.sso = token
```