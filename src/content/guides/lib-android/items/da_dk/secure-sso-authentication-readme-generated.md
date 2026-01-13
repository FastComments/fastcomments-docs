Implementer sikker godkendelse for dine brugere:

```kotlin
// Opret brugerdata (ideelt set på din server)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generer SSO-token (bør udføres på serversiden!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Tilføj til config
config.sso = token
```