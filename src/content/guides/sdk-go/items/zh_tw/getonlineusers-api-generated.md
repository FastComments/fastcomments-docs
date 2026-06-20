目前在線的頁面觀眾：目前其 websocket 會話已訂閱該頁面的人員。
回傳 anonCount + totalCount（房間範圍內的訂閱者，包括我們不逐一列舉的匿名檢視者）。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼（伺服器端已清理）。 |
| afterName | string | query | 否 | 游標：傳入上一個回應的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝用：傳入上一個回應的 nextAfterUserId。當設定 afterName 時此欄位為必填，以免在名稱相同時遺失條目。 |

## 回應

回傳：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## 範例

[inline-code-attrs-start title = 'GetOnlineUsers 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 頁面 URL 識別碼（伺服器端清理）。
	afterName := "afterName_example" // string | 游標：傳入上一個回應的 nextAfterName。（可選）
	afterUserId := "afterUserId_example" // string | 游標決勝項：傳入上一個回應的 nextAfterUserId。當設定 afterName 時為必填，以免在名稱相同時遺失條目。（可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOnlineUsers` 的回應：PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]

---