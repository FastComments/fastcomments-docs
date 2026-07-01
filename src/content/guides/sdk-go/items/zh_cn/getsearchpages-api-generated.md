## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| value | string | query | No |  |
| sso | string | query | No |  |

## 响应

返回：[`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_page_search_response.go)

## 示例

[inline-code-attrs-start title = 'GetSearchPages 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	value := "value_example" // string |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchPages(context.Background()).TenantId(tenantId).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "调用 `ModerationAPI.GetSearchPages`` 时出错: %v\n", err)
		fmt.Fprintf(os.Stderr, "完整的 HTTP 响应: %v\n", r)
	}
	// 来自 `GetSearchPages` 的响应: ModerationPageSearchResponse
	fmt.Fprintf(os.Stdout, "来自 `ModerationAPI.GetSearchPages` 的响应: %v\n", resp)
}
[inline-code-end]