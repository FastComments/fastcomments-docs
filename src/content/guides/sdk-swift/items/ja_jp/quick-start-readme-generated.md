### パブリック API の使用

```swift
import FastCommentsSwift

// ページのコメントを取得
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

### 認証済み API の使用

```swift
import FastCommentsSwift

// 共有設定に API キーを設定 (x-api-key ヘッダーとして送信されます)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// 認証済み API を使用してコメントを取得
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

### モデレーション API の使用

```swift
import FastCommentsSwift

// モデレーションメソッドは、実行モデレーター用の `sso` トークンで認可されます
// (FastCommentsSSO で生成し、上記の SSO セクションを参照してください)。
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

### 認証に SSO を使用

#### セキュア SSO（本番環境推奨）

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// セキュア SSO ユーザーデータを作成 (サーバー側のみ!)
let userData = SecureSSOUserData(
    id: "user-123",              // User ID
    email: "user@example.com",   // Email
    username: "johndoe",         // Username
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// SSO トークンを生成
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // このトークンをフロントエンドに渡して認証に使用
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### シンプル SSO（開発/テスト用）

```swift
import FastCommentsSwift

// シンプル SSO ユーザーデータを作成 (API キーは不要)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// シンプル SSO トークンを生成
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```