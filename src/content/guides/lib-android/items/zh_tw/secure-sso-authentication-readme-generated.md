為您的使用者實作安全的身份驗證：

```kotlin
// 建立使用者資料（理想情況下在您的伺服器上）
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// 產生 SSO token（應在伺服器端完成！）
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// 新增至 config
config.sso = token
```