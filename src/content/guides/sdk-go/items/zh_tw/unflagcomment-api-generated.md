## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 回應

回傳: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_response.go)

## 範例

[inline-code-attrs-start title = 'UnFlagComment 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (可選)
	anonUserId := "anonUserId_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnFlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnFlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `UnFlagComment` 的回應：FlagCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnFlagComment`: %v\n", resp)
}
[inline-code-end]