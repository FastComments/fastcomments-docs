为您的用户实现安全身份验证：

```kotlin
// 创建用户数据（理想情况下在您的服务器上）
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// 生成 SSO 令牌（应在服务器端完成！）
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// 添加到配置
config.sso = token
```