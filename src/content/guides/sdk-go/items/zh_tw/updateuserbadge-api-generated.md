## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | Yes |  |
| id | string | 路徑 | Yes |  |

## 回應

回傳: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_success_response.go)

## 範例

[inline-code-attrs-start title = 'UpdateUserBadge 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateUserBadgeParams := *openapiclient.NewUpdateUserBadgeParams() // UpdateUserBadgeParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateUserBadge(context.Background(), id).TenantId(tenantId).UpdateUserBadgeParams(updateUserBadgeParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateUserBadge` 的回應：APIEmptySuccessResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateUserBadge`: %v\n", resp)
}
[inline-code-end]