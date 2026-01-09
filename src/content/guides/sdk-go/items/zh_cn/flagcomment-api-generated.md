## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 响应

返回: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_200_response.go)

## 示例

[inline-code-attrs-start title = 'FlagComment 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (可选)
	anonUserId := "anonUserId_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.FlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.FlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `FlagComment` 的响应: FlagComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.FlagComment`: %v\n", resp)
}
[inline-code-end]

---