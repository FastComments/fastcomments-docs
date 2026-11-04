FastCommentsは3つの認証モードをサポートしています:

1. **匿名** -- SSOトークンなし。ユーザーにはセッションベースの識別情報が付与されます
2. **シンプルSSO** -- デモやテスト用のクライアントサイドトークン(安全ではありません)
3. **セキュアSSO** -- 本番環境向けのサーバー署名トークン

### シンプルSSO

デモやローカルテストに便利です。シンプルSSOでは誰でも任意のユーザーになりすますことができるため、本番環境では使用しないでください。

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

`SimpleSSOUserData` はオプションのフィールドもサポートしています:

- `id` -- ユーザーID(未設定の場合はメールアドレスがデフォルトになります)
- `displayName` -- 別途表示名を指定
- `displayLabel` -- 名前の横に表示されるカスタムラベル(例: "VIP")
- `websiteUrl` -- ユーザー名に付くリンク
- `locale` -- ロケールコード
- `isProfileActivityPrivate` -- プロフィールアクティビティを非表示にする(デフォルトはtrue)

### セキュアSSO

本番環境では、バックエンドがAPIシークレットを使用して署名済みのSSOトークンを生成します。iOSアプリはこのトークンをサーバーから取得し、設定に渡します。

**バックエンド側**(FastComments Swift SDKまたは任意の言語を使用):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// このトークンをAPI経由でiOSアプリに返します
```

**iOSアプリ側:**

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
            // バックエンドからトークンを取得します
            let token = try? await fetchSSOTokenFromYourBackend()
            // トークンを含む新しい設定を作成するか、ロード前に設定します
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` は追加のフィールドをサポートしています:

- `optedInNotifications` -- メール通知のオプトイン
- `displayLabel` -- カスタムラベル
- `displayName` -- 表示名
- `websiteUrl` -- ウェブサイトURL
- `groupIds` -- グループメンバーシップ
- `isAdmin` -- 管理者権限
- `isModerator` -- モデレーター権限
- `isProfileActivityPrivate` -- プロフィールのプライバシー

---
---