Implementa un'autenticazione sicura per i tuoi utenti:

```kotlin
// Crea i dati utente (idealmente sul tuo server)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Genera il token SSO (dovrebbe essere fatto lato server!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Aggiungi a config
config.sso = token
```