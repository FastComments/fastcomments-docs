## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetPendingWebhookEvents'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (по избор)
	externalId := "externalId_example" // string |  (по избор)
	eventType := "eventType_example" // string |  (по избор)
	type_ := "type__example" // string |  (по избор)
	domain := "domain_example" // string |  (по избор)
	attemptCountGT := float64(1.2) // float64 |  (по избор)
	skip := float64(1.2) // float64 |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetPendingWebhookEvents`: GetPendingWebhookEventsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]