Implemente la autenticación segura para sus usuarios:

```kotlin
// Crear datos de usuario (idealmente en su servidor)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generar token SSO (¡debe hacerse del lado del servidor!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Agregar a la configuración
config.sso = token
```