```bash
go get github.com/fastcomments/fastcomments-go
```

### API クライアントの使用方法

#### Public API（認証不要）

PublicAPI は、認証を必要としない公開エンドポイントへのアクセスを提供します:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // Get comments using PublicAPI
    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### Default API（APIキーが必要）

DefaultAPI は API キーを使用した認証が必要です:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // Create authenticated context with API key
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Get comments using authenticated DefaultAPI
    response, httpResp, err := apiClient.DefaultAPI.GetComments(auth).
        TenantId("your-tenant-id").
        UrlId("your-page-url-id").
        Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### Moderation API（モデレーターダッシュボード）

ModerationAPI はモデレーターダッシュボードを支える API です。コメントの一覧取得、件数取得、検索、エクスポート、モデレーション操作（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票、スレッドの再開/クローズ）、バン（コメント禁止、取り消し、事前バンの概要、バンのステータスと設定、バンされたユーザー数）、およびバッジと信頼度（バッジの付与/削除、手動バッジ、信頼度の取得/設定、ユーザー内部プロファイル）を提供します。すべての Moderation メソッドは SSO 認証されたモデレーター向けに `sso` パラメータを受け取ります:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // List comments for moderation using ModerationAPI
    response, httpResp, err := apiClient.ModerationAPI.GetApiComments(
        context.Background(),
    ).Sso("your-sso-token").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```