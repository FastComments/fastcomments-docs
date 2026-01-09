## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |
| order | string | query | Ні |  |
| after | number | query | Ні |  |
| before | number | query | Ні |  |

## Відповідь

Повертає: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetAuditLogs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (необов'язково)
	skip := float64(1.2) // float64 |  (необов'язково)
	order := openapiclient.SORT_DIR("ASC") // SORTDIR |  (необов'язково)
	after := float64(1.2) // float64 |  (необов'язково)
	before := float64(1.2) // float64 |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAuditLogs(context.Background()).TenantId(tenantId).Limit(limit).Skip(skip).Order(order).After(after).Before(before).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAuditLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAuditLogs`: GetAuditLogs200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]