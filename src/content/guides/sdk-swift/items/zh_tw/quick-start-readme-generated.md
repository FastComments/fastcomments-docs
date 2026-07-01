### 使用公開 API

```swift
import FastCommentsSwift

// 取得頁面的評論
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

### 使用已驗證 API

```swift
import FastCommentsSwift

// 在共享設定中配置您的 API 金鑰（作為 x-api-key 標頭傳送）
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// 使用已驗證的 API 取得評論
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

### 使用審核 API

```swift
import FastCommentsSwift

// 審核方法需要使用 `sso` 令牌授權給執行的審核者
// （使用 FastCommentsSSO 產生，請參閱上面的 SSO 章節。）
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

### 使用 SSO 進行驗證

#### 安全 SSO（建議於正式環境使用）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// 建立安全 SSO 使用者資料（僅限伺服器端！）
let userData = SecureSSOUserData(
    id: "user-123",              // 使用者 ID
    email: "user@example.com",   // 電子郵件
    username: "johndoe",         // 使用者名稱
    avatar: "https://example.com/avatar.jpg" // 大頭貼 URL
)

// Generate SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // 將此令牌傳遞給前端以進行驗證
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### 簡易 SSO（開發/測試用）

```swift
import FastCommentsSwift

// 建立簡易 SSO 使用者資料（不需 API 金鑰）
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generate simple SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```