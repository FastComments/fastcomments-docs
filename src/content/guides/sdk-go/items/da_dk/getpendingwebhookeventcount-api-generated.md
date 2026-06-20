---
## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nej |  |
| externalId | string | query | Nej |  |
| eventType | string | query | Nej |  |
| type | string | query | Nej |  |
| domain | string | query | Nej |  |
| attemptCountGT | number | query | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetPendingWebhookEventCount Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (valgfri)
	externalId := "externalId_example" // string |  (valgfri)
	eventType := "eventType_example" // string |  (valgfri)
	type_ := "type__example" // string |  (valgfri)
	domain := "domain_example" // string |  (valgfri)
	attemptCountGT := float64(1.2) // float64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetPendingWebhookEventCount`: GetPendingWebhookEventCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]

---