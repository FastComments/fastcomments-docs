---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_page_api_response.go)

## 示例

[inline-code-attrs-start title = 'AddPage 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createAPIPageData := *openapiclient.NewCreateAPIPageData("Title_example", "Url_example", "UrlId_example") // CreateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddPage(context.Background()).TenantId(tenantId).CreateAPIPageData(createAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `AddPage` 的响应: AddPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddPage`: %v\n", resp)
}
[inline-code-end]

---