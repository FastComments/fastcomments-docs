FastComments 支持三种认证模式：

1. **Anonymous** -- 无 SSO 令牌；用户获得基于会话的身份
2. **Simple SSO** -- 客户端令牌，用于演示和测试（不安全）
3. **Secure SSO** -- 服务器签名的令牌，用于生产环境

### Simple SSO

适用于演示和本地测试。任何人都可以使用 Simple SSO 冒充任何用户，因此不要在生产环境中使用它。

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

`SimpleSSOUserData` 也支持可选字段：

- `id` -- user ID（如果未设置则默认为 email）
- `displayName` -- 单独的显示名称
- `displayLabel` -- 在姓名旁显示的自定义标签（例如 "VIP"）
- `websiteUrl` -- 用户姓名上的链接
- `locale` -- 区域代码
- `isProfileActivityPrivate` -- 隐藏个人资料活动（默认值为 true）

### Secure SSO

在生产环境中，你的后端使用你的 API 密钥生成签名的 SSO 令牌。iOS 应用从你的服务器获取该令牌并将其传递给配置。

**在你的后端**（使用 FastComments Swift SDK 或任意语言）:

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// 通过你的 API 将此令牌返回给你的 iOS 应用
```

**在你的 iOS 应用中：**

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
            // 从你的后端获取令牌
            let token = try? await fetchSSOTokenFromYourBackend()
            // 使用令牌创建新的配置，或在加载前设置它
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` 支持额外字段：

- `optedInNotifications` -- email 通知的选择加入
- `displayLabel` -- 自定义标签
- `displayName` -- 显示名称
- `websiteUrl` -- 网站 URL
- `groupIds` -- 组成员身份
- `isAdmin` -- 管理员权限
- `isModerator` -- 版主权限
- `isProfileActivityPrivate` -- 个人资料隐私

---
---