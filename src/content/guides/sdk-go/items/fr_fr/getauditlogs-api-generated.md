## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| limit | number | query | Non |  |
| skip | number | query | Non |  |
| order | string | query | Non |  |
| after | number | query | Non |  |
| before | number | query | Non |  |

## Réponse

Renvoie : [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetAuditLogs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (optionnel)
	skip := float64(1.2) // float64 |  (optionnel)
	order := openapiclient.SORT_DIR("ASC") // SORTDIR |  (optionnel)
	after := float64(1.2) // float64 |  (optionnel)
	before := float64(1.2) // float64 |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAuditLogs(context.Background()).TenantId(tenantId).Limit(limit).Skip(skip).Order(order).After(after).Before(before).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAuditLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetAuditLogs`: GetAuditLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]