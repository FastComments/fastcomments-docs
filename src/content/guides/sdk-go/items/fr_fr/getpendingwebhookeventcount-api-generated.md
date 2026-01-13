## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Non |  |
| externalId | string | query | Non |  |
| eventType | string | query | Non |  |
| type | string | query | Non |  |
| domain | string | query | Non |  |
| attemptCountGT | number | query | Non |  |

## Réponse

Retourne : [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (optionnel)
	externalId := "externalId_example" // string |  (optionnel)
	eventType := "eventType_example" // string |  (optionnel)
	type_ := "type__example" // string |  (optionnel)
	domain := "domain_example" // string |  (optionnel)
	attemptCountGT := float64(1.2) // float64 |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetPendingWebhookEventCount`: GetPendingWebhookEventCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]