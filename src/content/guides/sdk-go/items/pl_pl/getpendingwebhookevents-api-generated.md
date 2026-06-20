## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Odpowiedź

Zwraca: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetPendingWebhookEvents'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (opcjonalne)
	externalId := "externalId_example" // string |  (opcjonalne)
	eventType := "eventType_example" // string |  (opcjonalne)
	type_ := "type__example" // string |  (opcjonalne)
	domain := "domain_example" // string |  (opcjonalne)
	attemptCountGT := float64(1.2) // float64 |  (opcjonalne)
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetPendingWebhookEvents`: GetPendingWebhookEventsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]