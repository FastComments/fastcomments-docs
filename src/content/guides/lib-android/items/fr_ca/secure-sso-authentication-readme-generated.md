Implémentez une authentification sécurisée pour vos utilisateurs :

```kotlin
// Créez les données utilisateur (idéalement sur votre serveur)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Générez le jeton SSO (doit être fait côté serveur !)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Ajoutez à la configuration
config.sso = token
```