FastComments 支持三种身份验证模式：

1. **匿名** -- 无 SSO 令牌；用户获得基于会话的身份
2. **简单 SSO** -- 用于演示和测试的客户端令牌（不安全）
3. **安全 SSO** -- 用于生产环境的服务器签名令牌

### 简单 SSO

适用于演示和本地测试。任何人都可以通过简单 SSO 冒充任何用户，因此请勿在生产环境中使用。

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

`SimpleSSOUserData` 还支持可选字段：

- `id` -- 用户 ID（未设置时默认为邮箱）
- `displayName` -- 单独的显示名称
- `displayLabel` -- 显示在名称旁边的自定义标签（例如 "VIP"）
- `websiteUrl` -- 用户名称上的链接
- `locale` -- 区域设置代码
- `isProfileActivityPrivate` -- 隐藏个人资料活动（默认为 true）

### 安全 SSO

在生产环境中，您的后端使用 API 密钥生成签名的 SSO 令牌。iOS 应用从您的服务器获取此令牌并将其传递给配置。

**在您的后端**（使用 FastComments Swift SDK 或任何语言）：

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// 通过您的 API 将此令牌返回给您的 iOS 应用
```

**在您的 iOS 应用中：**

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
            // 从您的后端获取令牌
            let token = try? await fetchSSOTokenFromYourBackend()
            // 使用该令牌创建新配置，或在加载前设置
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` 支持其他字段：

- `optedInNotifications` -- 邮件通知选择加入
- `displayLabel` -- 自定义标签
- `displayName` -- 显示名称
- `websiteUrl` -- 网站 URL
- `groupIds` -- 群组成员身份
- `isAdmin` -- 管理员权限
- `isModerator` -- 版主权限
- `isProfileActivityPrivate` -- 个人资料隐私