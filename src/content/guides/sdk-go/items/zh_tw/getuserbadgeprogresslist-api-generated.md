---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_progress_list_response.go)

## 範例

[inline-code-attrs-start title = 'GetUserBadgeProgressList 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (選用)
	limit := float64(1.2) // float64 |  (選用)
	skip := float64(1.2) // float64 |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressList(context.Background()).TenantId(tenantId).UserId(userId).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserBadgeProgressList`: APIGetUserBadgeProgressListResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressList`: %v\n", resp)
}
[inline-code-end]

---