### パブリックAPIの使用

```swift
import FastCommentsSwift

// APIクライアントを作成
let publicApi = PublicAPI()

// ページのコメントを取得
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

### 認証済みAPIの使用

```swift
import FastCommentsSwift

// APIキーで設定を作成
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// 認証済みAPIを使ってコメントを取得
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

### SSOを使った認証

#### セキュアSSO（本番環境に推奨）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// セキュアSSOユーザーデータを作成（サーバー側のみ！）
let userData = SecureSSOUserData(
    id: "user-123",              // ユーザーID
    email: "user@example.com",   // メール
    username: "johndoe",         // ユーザー名
    avatar: "https://example.com/avatar.jpg" // アバターのURL
)

// SSOトークンを生成
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // このトークンをフロントエンドに渡して認証に使用する
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### シンプルSSO（開発/テスト用）

```swift
import FastCommentsSwift

// シンプルSSOユーザーデータを作成（APIキー不要）
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// シンプルSSOトークンを生成
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```