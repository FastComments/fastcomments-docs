Implemente autenticação segura para seus usuários:

```kotlin
// Criar dados do usuário (idealmente no seu servidor)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Gerar token SSO (deve ser feito no servidor!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Adicionar à configuração
config.sso = token
```