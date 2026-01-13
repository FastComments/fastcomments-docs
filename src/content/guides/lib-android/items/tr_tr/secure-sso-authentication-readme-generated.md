Kullanıcılarınız için güvenli kimlik doğrulaması uygulayın:

```kotlin
// Kullanıcı verisini oluşturun (tercihen sunucunuzda)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// SSO jetonu oluşturun (sunucu tarafında yapılmalıdır!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Yapılandırmaya ekle
config.sso = token
```