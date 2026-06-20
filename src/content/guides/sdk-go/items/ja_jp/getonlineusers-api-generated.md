現在ページにオンラインの閲覧者: 現在そのページにWebSocketセッションを購読しているユーザー。
anonCount と totalCount を返します（ルーム全体の購読者数、列挙しない匿名閲覧者も含む）。

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページのURL識別子（サーバー側でクリーンされます）。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されているときに、名前が同一のエントリが除外されないように必須です。 |

## レスポンス

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## 例

[inline-code-attrs-start title = 'GetOnlineUsers の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | ページのURL識別子（サーバー側でクリーンされます）。
	afterName := "afterName_example" // string | カーソル: 前のレスポンスの nextAfterName を渡します。 (任意)
	afterUserId := "afterUserId_example" // string | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合に、名前が同一のエントリが除外されないように必要です。 (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOnlineUsers` のレスポンス: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]

---