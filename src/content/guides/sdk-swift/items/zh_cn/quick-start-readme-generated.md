---
### 使用公共 API

```swift
import FastCommentsSwift

// 创建 API 客户端
let publicApi = PublicAPI()

// 获取页面的评论
do {
    let response = try await publicApi.getCommentsPublic(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Found \(response.comments?.count ?? 0) comments")
    for comment in response.comments ?? [] {
        print("Comment: \(comment.comment ?? "")")
    }
} catch {
    print("Error fetching comments: \(error)")
}
```

### 使用已认证的 API

```swift
import FastCommentsSwift

// 使用 API 密钥创建配置
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// 使用已认证的 API 获取评论
do {
    let response = try await defaultApi.getComments(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### 使用 SSO 进行认证

#### 安全 SSO（推荐用于生产环境）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// 创建安全 SSO 用户数据（仅限服务器端！）
let userData = SecureSSOUserData(
    id: "user-123",              // 用户 ID
    email: "user@example.com",   // 电子邮件
    username: "johndoe",         // 用户名
    avatar: "https://example.com/avatar.jpg" // 头像 URL
)

// 生成 SSO 令牌
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // 将此令牌传递到前端以进行认证
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### 简单 SSO（用于开发/测试）

```swift
import FastCommentsSwift

// 创建简单 SSO 用户数据（无需 API 密钥）
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// 生成简单 SSO 令牌
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```
---