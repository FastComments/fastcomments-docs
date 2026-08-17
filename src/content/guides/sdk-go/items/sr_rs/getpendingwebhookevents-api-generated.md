## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Response

Враћа: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_response.go)

## Example

[inline-code-attrs-start title = 'GetPendingWebhookEvents Пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (опционално)
	externalId := "externalId_example" // string |  (опционално)
	eventType := "eventType_example" // string |  (опционално)
	type_ := "type__example" // string |  (опционално)
	domain := "domain_example" // string |  (опционално)
	attemptCountGT := float64(1.2) // float64 |  (опционално)
	skip := float64(1.2) // float64 |  (опционално)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetPendingWebhookEvents`: GetPendingWebhookEventsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]