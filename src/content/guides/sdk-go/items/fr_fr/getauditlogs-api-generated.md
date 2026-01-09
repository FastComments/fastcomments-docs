## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | requête | Oui |  |
| limit | number | requête | Non |  |
| skip | number | requête | Non |  |
| order | string | requête | Non |  |
| after | number | requête | Non |  |
| before | number | requête | Non |  |

## Réponse

Renvoie : [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetAuditLogs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// réponse de `GetAuditLogs`: GetAuditLogs200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]