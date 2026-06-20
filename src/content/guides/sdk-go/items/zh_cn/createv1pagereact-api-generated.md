## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| title | string | 查询 | 否 |  |

## 响应

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## 示例

[inline-code-attrs-start title = 'CreateV1PageReact 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	title := "title_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateV1PageReact(context.Background(), tenantId).UrlId(urlId).Title(title).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `CreateV1PageReact` 的响应：CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateV1PageReact`: %v\n", resp)
}
[inline-code-end]