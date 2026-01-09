## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| contextUserId | string | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| isLive | boolean | query | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## 示例

[inline-code-attrs-start title = 'UpdateComment 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	id := "id_example" // string | 
	body := PickAPICommentUpdatableCommentFields(987) // PickAPICommentUpdatableCommentFields | 
	contextUserId := "contextUserId_example" // string |  （可选）
	doSpamCheck := true // bool |  （可选）
	isLive := true // bool |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateComment(context.Background(), id).TenantId(tenantId).Body(body).ContextUserId(contextUserId).DoSpamCheck(doSpamCheck).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateComment`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateComment`: %v\n", resp)
}
[inline-code-end]