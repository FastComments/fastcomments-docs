## Параметри

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (необов'язковий)
	externalId := "externalId_example" // string |  (необов'язковий)
	eventType := "eventType_example" // string |  (необов'язковий)
	type_ := "type__example" // string |  (необов'язковий)
	domain := "domain_example" // string |  (необов'язковий)
	attemptCountGT := float64(1.2) // float64 |  (необов'язковий)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetPendingWebhookEventCount`: GetPendingWebhookEventCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]