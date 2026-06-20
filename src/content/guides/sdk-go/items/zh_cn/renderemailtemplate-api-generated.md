## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| locale | string | query | 否 |  |

## 响应

返回: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_render_email_template_response.go)

## 示例

[inline-code-attrs-start title = 'RenderEmailTemplate 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	renderEmailTemplateBody := *openapiclient.NewRenderEmailTemplateBody("EmailTemplateId_example", "Ejs_example") // RenderEmailTemplateBody | 
	locale := "locale_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.RenderEmailTemplate(context.Background()).TenantId(tenantId).RenderEmailTemplateBody(renderEmailTemplateBody).Locale(locale).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.RenderEmailTemplate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `RenderEmailTemplate` 的响应: RenderEmailTemplateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.RenderEmailTemplate`: %v\n", resp)
}
[inline-code-end]