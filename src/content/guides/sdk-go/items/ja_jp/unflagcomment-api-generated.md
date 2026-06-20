## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## レスポンス

戻り値: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_response.go)

## 例

[inline-code-attrs-start title = 'UnFlagComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (オプション)
	anonUserId := "anonUserId_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnFlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnFlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UnFlagComment` のレスポンス: FlagCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnFlagComment`: %v\n", resp)
}
[inline-code-end]