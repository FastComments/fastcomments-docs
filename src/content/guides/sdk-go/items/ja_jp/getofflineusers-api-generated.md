---
現在オンラインではない、そのページの過去のコメント投稿者。displayNameでソート。
/users/online を使い尽くした後に「メンバー」セクションをレンダリングするためにこれを使用してください。
commenterNameに対するカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを辿ります。
afterName から先を $gt で前方にインデックス、$skip コストはありません。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Response

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Example

[inline-code-attrs-start title = 'GetOfflineUsers の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Page URL identifier (cleaned server-side).
	afterName := "afterName_example" // string | カーソル: 前のレスポンスからの nextAfterName を渡します。 (optional)
	afterUserId := "afterUserId_example" // string | カーソルのタイブレーカー: 前のレスポンスからの nextAfterUserId を渡します。afterName が設定されている場合、名前の同一性でエントリが落ちないように必要です。 (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---