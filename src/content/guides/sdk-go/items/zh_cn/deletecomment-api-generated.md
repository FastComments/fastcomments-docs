## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| id | string | 路径 | 是 |  |
| contextUserId | string | 查询 | 否 |  |
| isLive | boolean | 查询 | 否 |  |

## 响应

返回: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_200_response.go)

## 示例

[inline-code-attrs-start title = 'DeleteComment 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	contextUserId := "contextUserId_example" // string |  (可选)
	isLive := true // bool |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteComment(context.Background(), id).TenantId(tenantId).ContextUserId(contextUserId).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `DeleteComment` 的响应: DeleteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteComment`: %v\n", resp)
}
[inline-code-end]