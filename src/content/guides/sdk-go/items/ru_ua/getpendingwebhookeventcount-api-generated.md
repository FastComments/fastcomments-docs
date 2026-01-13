## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Нет |  |
| externalId | string | query | Нет |  |
| eventType | string | query | Нет |  |
| type | string | query | Нет |  |
| domain | string | query | Нет |  |
| attemptCountGT | number | query | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (необязательно)
	externalId := "externalId_example" // string |  (необязательно)
	eventType := "eventType_example" // string |  (необязательно)
	type_ := "type__example" // string |  (необязательно)
	domain := "domain_example" // string |  (необязательно)
	attemptCountGT := float64(1.2) // float64 |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetPendingWebhookEventCount`: GetPendingWebhookEventCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]