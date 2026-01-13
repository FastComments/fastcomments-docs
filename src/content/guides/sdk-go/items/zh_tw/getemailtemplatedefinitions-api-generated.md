## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_definitions_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetEmailTemplateDefinitions 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplateDefinitions(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplateDefinitions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetEmailTemplateDefinitions` 的回應： GetEmailTemplateDefinitions200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplateDefinitions`: %v\n", resp)
}
[inline-code-end]

---