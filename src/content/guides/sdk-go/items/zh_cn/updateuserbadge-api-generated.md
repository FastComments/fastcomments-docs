## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回：[`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_success_response.go)

## 示例

[inline-code-attrs-start title = 'UpdateUserBadge 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 租户 ID
	id := "id_example" // string | 标识
	updateUserBadgeParams := *openapiclient.NewUpdateUserBadgeParams() // UpdateUserBadgeParams | 更新用户徽章参数

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateUserBadge(context.Background(), id).TenantId(tenantId).UpdateUserBadgeParams(updateUserBadgeParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `UpdateUserBadge` 的响应：APIEmptySuccessResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateUserBadge`: %v\n", resp)
}
[inline-code-end]

---