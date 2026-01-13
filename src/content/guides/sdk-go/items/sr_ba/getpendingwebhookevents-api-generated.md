## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Ne |  |
| externalId | string | query | Ne |  |
| eventType | string | query | Ne |  |
| type | string | query | Ne |  |
| domain | string | query | Ne |  |
| attemptCountGT | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_200_response.go)

## Primjer

[inline-code-attrs-start title = 'GetPendingWebhookEvents Primjer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (neobavezno)
	externalId := "externalId_example" // string |  (neobavezno)
	eventType := "eventType_example" // string |  (neobavezno)
	type_ := "type__example" // string |  (neobavezno)
	domain := "domain_example" // string |  (neobavezno)
	attemptCountGT := float64(1.2) // float64 |  (neobavezno)
	skip := float64(1.2) // float64 |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetPendingWebhookEvents`: GetPendingWebhookEvents200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]

---