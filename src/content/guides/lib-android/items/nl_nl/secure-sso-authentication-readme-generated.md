Implementeer veilige authenticatie voor uw gebruikers:

```kotlin
// Maak gebruikersgegevens aan (bij voorkeur op uw server)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Genereer SSO-token (moet aan de serverzijde gebeuren!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Voeg toe aan config
config.sso = token
```