---
租戶的批次使用者資訊。給定 userIds，回傳來自 User / SSOUser 的顯示資訊。
由評論小工具使用，以豐富剛透過 presence 事件出現的使用者。
沒有頁面上下文：隱私以一致的方式強制執行（私人檔案會被遮蔽）。

## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | 以逗號分隔的 userIds。 |

## 回應

回傳: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## 範例

[inline-code-attrs-start title = 'GetUsersInfo 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | Comma-delimited userIds.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 回應自 `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]

---