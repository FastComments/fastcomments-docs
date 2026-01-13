## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | 是 |  |
| tenantId | string | query | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## 範例

[inline-code-attrs-start title = 'DeleteHashTag 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  （可選）
	deleteHashTagRequest := *openapiclient.NewDeleteHashTagRequest() // DeleteHashTagRequest |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteHashTag(context.Background(), tag).TenantId(tenantId).DeleteHashTagRequest(deleteHashTagRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `DeleteHashTag` 的回應: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteHashTag`: %v\n", resp)
}
[inline-code-end]