---
## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Απόκριση

Επιστρέφει: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetPendingWebhookEvents'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (προαιρετικό)
	externalId := "externalId_example" // string |  (προαιρετικό)
	eventType := "eventType_example" // string |  (προαιρετικό)
	type_ := "type__example" // string |  (προαιρετικό)
	domain := "domain_example" // string |  (προαιρετικό)
	attemptCountGT := float64(1.2) // float64 |  (προαιρετικό)
	skip := float64(1.2) // float64 |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetPendingWebhookEvents`: GetPendingWebhookEventsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]

---