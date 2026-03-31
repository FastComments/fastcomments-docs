---
FastComments は 3 つの認証モードをサポートしています:

1. **Anonymous** -- SSO トークンなし; ユーザーはセッションベースの識別を取得します
2. **Simple SSO** -- デモやテスト用のクライアント側トークン（安全ではありません）
3. **Secure SSO** -- 本番向けのサーバー署名トークン

### Simple SSO

デモやローカルテストに便利です。Simple SSO では誰でも任意のユーザーをなりすますことができるため、本番環境では使用しないでください。

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

`SimpleSSOUserData` は次のオプションフィールドもサポートします:

- `id` -- ユーザー ID（未設定の場合は email がデフォルト）
- `displayName` -- 個別の表示名
- `displayLabel` -- 名前の横に表示されるカスタムラベル（例: "VIP"）
- `websiteUrl` -- ユーザー名に付くリンク
- `locale` -- ロケールコード
- `isProfileActivityPrivate` -- プロファイルアクティビティを非表示にする（デフォルトは true）

### Secure SSO

本番環境では、バックエンドが API シークレットを使って署名付き SSO トークンを生成します。iOS アプリはサーバーからこのトークンを取得し、config に渡します。

**バックエンド側**（FastComments Swift SDK または任意の言語を使用）:

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// このトークンを API 経由で iOS アプリに返します
```

**iOS アプリ内で:**

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
            // バックエンドからトークンを取得
            let token = try? await fetchSSOTokenFromYourBackend()
            // トークンで新しい config を作成するか、ロード前に設定する
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` は追加のフィールドをサポートします:

- `optedInNotifications` -- メール通知のオプトイン
- `displayLabel` -- カスタムラベル
- `displayName` -- 表示名
- `websiteUrl` -- ウェブサイトの URL
- `groupIds` -- グループ所属
- `isAdmin` -- 管理者権限
- `isModerator` -- モデレーター権限
- `isProfileActivityPrivate` -- プロフィールの非公開設定

---
---