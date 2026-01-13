Υλοποιήστε ασφαλή πιστοποίηση για τους χρήστες σας:

```kotlin
// Δημιουργήστε δεδομένα χρήστη (κατά προτίμηση στον διακομιστή σας)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generate SSO token (should be done server-side!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Προσθέστε στο config
config.sso = token
```