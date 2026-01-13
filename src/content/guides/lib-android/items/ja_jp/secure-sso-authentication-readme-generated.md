ユーザーのために安全な認証を実装する:

```kotlin
// ユーザーデータを作成（理想的にはサーバー側で）
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// SSOトークンを生成（サーバー側で行うべき！）
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// 設定に追加
config.sso = token
```