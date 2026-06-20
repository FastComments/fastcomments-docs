## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 响应

返回：[`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_success_response.go)

## 示例

[inline-code-attrs-start title = 'DeleteUserBadge 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字符串 | 
	id := "id_example" // 字符串 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteUserBadge(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `DeleteUserBadge` 的响应：APIEmptySuccessResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteUserBadge`: %v\n", resp)
}
[inline-code-end]

---