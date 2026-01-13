Implementieren Sie eine sichere Authentifizierung für Ihre Benutzer:

```kotlin
// Benutzerdaten erstellen (idealerweise auf Ihrem Server)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// SSO-Token generieren (sollte serverseitig erfolgen!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Zur Konfiguration hinzufügen
config.sso = token
```