## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nein |  |
| externalId | string | query | Nein |  |
| eventType | string | query | Nein |  |
| type | string | query | Nein |  |
| domain | string | query | Nein |  |
| attemptCountGT | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (optional)
	externalId := "externalId_example" // string |  (optional)
	eventType := "eventType_example" // string |  (optional)
	type_ := "type__example" // string |  (optional)
	domain := "domain_example" // string |  (optional)
	attemptCountGT := float64(1.2) // float64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetPendingWebhookEventCount`: GetPendingWebhookEventCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]

---