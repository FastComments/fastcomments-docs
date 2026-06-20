## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| locale | string | query | Не |  |

## Отговор

Връща: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_render_email_template_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за RenderEmailTemplate'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (незадължително)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.RenderEmailTemplate(context.Background()).TenantId(tenantId).RenderEmailTemplateBody(renderEmailTemplateBody).Locale(locale).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.RenderEmailTemplate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `RenderEmailTemplate`: RenderEmailTemplateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.RenderEmailTemplate`: %v\n", resp)
}
[inline-code-end]