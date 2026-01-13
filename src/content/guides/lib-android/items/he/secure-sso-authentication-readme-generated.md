הטמעת אימות מאובטח עבור המשתמשים שלך:

```kotlin
// צור נתוני משתמש (מומלץ לבצע זאת בשרת שלך)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// יצירת אסימון SSO (יש לבצע זאת בצד השרת!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// הוסף לקונפיג
config.sso = token
```