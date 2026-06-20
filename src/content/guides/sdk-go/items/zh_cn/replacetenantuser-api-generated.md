## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| updateComments | string | query | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## 示例

[inline-code-attrs-start title = 'ReplaceTenantUser 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	replaceTenantUserBody := *openapiclient.NewReplaceTenantUserBody("Username_example", "Email_example") // ReplaceTenantUserBody | 
	updateComments := "updateComments_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ReplaceTenantUser(context.Background(), id).TenantId(tenantId).ReplaceTenantUserBody(replaceTenantUserBody).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ReplaceTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `ReplaceTenantUser` 的响应: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ReplaceTenantUser`: %v\n", resp)
}
[inline-code-end]