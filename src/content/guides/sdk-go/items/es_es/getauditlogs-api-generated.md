## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Respuesta

Devuelve: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetAuditLogs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (opcional)
	skip := float64(1.2) // float64 |  (opcional)
	order := openapiclient.SORT_DIR("ASC") // SORTDIR |  (opcional)
	after := float64(1.2) // float64 |  (opcional)
	before := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAuditLogs(context.Background()).TenantId(tenantId).Limit(limit).Skip(skip).Order(order).After(after).Before(before).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAuditLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetAuditLogs`: GetAuditLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]