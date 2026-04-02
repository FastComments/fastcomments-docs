FastComments 支援三種驗證模式：

1. **匿名** -- 無 SSO 令牌；使用者會獲得基於會話的身分
2. **簡易 SSO** -- 用於展示與本機測試的用戶端令牌（不安全）
3. **安全 SSO** -- 適用於生產環境、由伺服器簽署的令牌

### 簡易 SSO

適用於展示和本機測試。任何人都可以使用簡易 SSO 偽裝成任何使用者，因此不要在生產環境中使用。

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData` also supports optional fields:

- `id` -- 使用者 ID（如果未設定則預設為電子郵件）
- `displayName` -- 分開的顯示名稱
- `displayLabel` -- 顯示在名稱旁的自訂標籤（例如「VIP」）
- `websiteUrl` -- 使用者名稱的連結
- `locale` -- 語系代碼
- `isProfileActivityPrivate` -- 隱藏個人檔案活動（預設為 true）

### 安全 SSO

在生產環境中，您的後端會使用 API 秘密產生已簽署的 SSO 令牌。iOS 應用程式會從您的伺服器取得該令牌並傳遞至設定中。

**在您的後端**（使用 FastComments Swift SDK 或任何語言）：

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Return this token to your iOS app via your API
```

**在您的 iOS 應用程式中：**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // Fetch the token from your backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Create a new config with the token, or set it before load
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` supports additional fields:

- `optedInNotifications` -- 選擇接收電子郵件通知
- `displayLabel` -- 自訂標籤
- `displayName` -- 顯示名稱
- `websiteUrl` -- 網站 URL
- `groupIds` -- 所屬群組
- `isAdmin` -- 管理員權限
- `isModerator` -- 版主權限
- `isProfileActivityPrivate` -- 個人檔案活動隱私

---
---