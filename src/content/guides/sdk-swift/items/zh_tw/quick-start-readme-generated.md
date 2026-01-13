### 使用公開 API

```swift
import FastCommentsSwift

// 建立 API 用戶端
let publicApi = PublicAPI()

// Fetch comments for a page
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

### 使用已驗證的 API

```swift
import FastCommentsSwift

// 使用 API 金鑰建立設定
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Fetch comments using authenticated API
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

### 使用 SSO 進行驗證

#### 安全 SSO（建議用於生產環境）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// 建立安全 SSO 使用者資料（僅限伺服器端！）
let userData = SecureSSOUserData(
    id: "user-123",              // 使用者 ID
    email: "user@example.com",   // 電子郵件
    username: "johndoe",         // 使用者名稱
    avatar: "https://example.com/avatar.jpg" // 頭像 URL
)

// 產生 SSO 令牌
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // 將此令牌傳給前端以進行驗證
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### 簡易 SSO（用於開發/測試）

```swift
import FastCommentsSwift

// 建立簡易 SSO 使用者資料（不需 API 金鑰）
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// 產生簡易 SSO 令牌
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```