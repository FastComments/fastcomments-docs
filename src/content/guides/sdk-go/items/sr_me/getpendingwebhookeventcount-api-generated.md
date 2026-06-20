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

## Odgovor

Vraća: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (opciono)
	externalId := "externalId_example" // string |  (opciono)
	eventType := "eventType_example" // string |  (opciono)
	type_ := "type__example" // string |  (opciono)
	domain := "domain_example" // string |  (opciono)
	attemptCountGT := float64(1.2) // float64 |  (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetPendingWebhookEventCount`: GetPendingWebhookEventCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]