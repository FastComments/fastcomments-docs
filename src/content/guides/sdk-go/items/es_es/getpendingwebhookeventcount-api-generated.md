## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Respuesta

Devuelve: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (opcional)
	externalId := "externalId_example" // string |  (opcional)
	eventType := "eventType_example" // string |  (opcional)
	type_ := "type__example" // string |  (opcional)
	domain := "domain_example" // string |  (opcional)
	attemptCountGT := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetPendingWebhookEventCount`: GetPendingWebhookEventCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]