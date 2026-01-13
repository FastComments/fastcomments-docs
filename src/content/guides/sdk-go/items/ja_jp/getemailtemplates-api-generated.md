## パラメータ

| 名称 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

返却値: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_templates_200_response.go)

## 例

[inline-code-attrs-start title = 'GetEmailTemplates の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplates(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplates``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetEmailTemplates` のレスポンス: GetEmailTemplates200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplates`: %v\n", resp)
}
[inline-code-end]

---