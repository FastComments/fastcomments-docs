過去在該頁面留言但目前不在線上的使用者。按 displayName 排序。
在用盡 /users/online 後使用此方法來呈現「成員」區段。
基於 commenterName 的游標分頁：伺服器會沿著部分 {tenantId, urlId, commenterName}
索引從 afterName 向前走動，使用 $gt，無 $skip 成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別符（伺服器端已清理）。 |
| afterName | string | query | 否 | 游標：傳入先前回應的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標平手決勝：傳入先前回應的 nextAfterUserId。當設定 afterName 時需要，以免名稱相同導致條目遺失。 |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## 範例

[inline-code-attrs-start title = 'GetOfflineUsers 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 頁面 URL 識別符（伺服器端已清理）。
	afterName := "afterName_example" // string | 游標：傳入先前回應的 nextAfterName。（可選）
	afterUserId := "afterUserId_example" // string | 游標平手決勝：傳入先前回應的 nextAfterUserId。當設定 afterName 時需要，以免名稱相同導致條目遺失。（可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOfflineUsers` 的回應：PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---