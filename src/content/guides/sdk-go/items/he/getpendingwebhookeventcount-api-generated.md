## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | לא |  |
| externalId | string | query | לא |  |
| eventType | string | query | לא |  |
| type | string | query | לא |  |
| domain | string | query | לא |  |
| attemptCountGT | number | query | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_event_count_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetPendingWebhookEventCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (אופציונלי)
	externalId := "externalId_example" // string |  (אופציונלי)
	eventType := "eventType_example" // string |  (אופציונלי)
	type_ := "type__example" // string |  (אופציונלי)
	domain := "domain_example" // string |  (אופציונלי)
	attemptCountGT := float64(1.2) // float64 |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEventCount(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEventCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetPendingWebhookEventCount`: GetPendingWebhookEventCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEventCount`: %v\n", resp)
}
[inline-code-end]