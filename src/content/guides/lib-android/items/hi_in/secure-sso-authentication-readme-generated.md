अपने उपयोगकर्ताओं के लिए सुरक्षित प्रमाणीकरण लागू करें:

```kotlin
// Create user data (ideally on your server)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// Generate SSO token (should be done server-side!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// Add to config
config.sso = token
```