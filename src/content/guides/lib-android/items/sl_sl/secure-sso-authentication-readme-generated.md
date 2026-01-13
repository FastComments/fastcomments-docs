Uvedite varno overjanje za vaše uporabnike:

```kotlin
// Ustvarite podatke o uporabniku (najbolje na vašem strežniku)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generirajte SSO žeton (to naj bo narejeno na strežniku!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Dodajte v config
config.sso = token
```