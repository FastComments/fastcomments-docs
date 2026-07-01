### 使用公共 API

```swift
import FastCommentsSwift

// 获取页面的评论
do {
    let response = try await PublicAPI.getCommentsPublic(
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

### 使用已认证 API

```swift
import FastCommentsSwift

// 在共享配置上设置您的 API 密钥（作为 x-api-key 头发送）
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// 使用已认证 API 获取评论
do {
    let response = try await DefaultAPI.getComments(
        tenantId: "your-tenant-id",
        options: .init(urlId: "page-url-id")
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### 使用审查 API

```swift
import FastCommentsSwift

// 审核方法需要使用 `sso` 令牌进行授权（使用 FastCommentsSSO 生成，参见上面的 SSO 部分）。
do {
    let response = try await ModerationAPI.getApiComments(
        options: .init(
            page: 0,
            count: 30,
            sso: ssoToken
        )
    )

    print("Found \(response.comments.count) comments to moderate")
    for comment in response.comments {
        print("Comment ID: \(comment.id), Text: \(comment.commentHTML)")
    }
} catch {
    print("Error: \(error)")
}
```

### 使用 SSO 进行身份验证

#### 安全 SSO（推荐用于生产环境）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// 创建安全的 SSO 用户数据（仅限服务器端！）
let userData = SecureSSOUserData(
    id: "user-123",              // User ID
    email: "user@example.com",   // Email
    username: "johndoe",         // Username
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// 生成 SSO 令牌
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // 将此令牌传递给前端进行身份验证
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### 简单 SSO（用于开发/测试）

```swift
import FastCommentsSwift

// 创建简单的 SSO 用户数据（无需 API 密钥）
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