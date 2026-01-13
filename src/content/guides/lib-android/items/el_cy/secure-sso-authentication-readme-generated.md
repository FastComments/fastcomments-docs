Εφαρμόστε ασφαλή αυθεντικοποίηση για τους χρήστες σας:

```kotlin
// Δημιουργία δεδομένων χρήστη (κατά προτίμηση στον διακομιστή σας)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Δημιουργία SSO token (θα πρέπει να γίνεται στον διακομιστή!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Προσθήκη στη διαμόρφωση
config.sso = token
```