## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 否 |  |
| externalId | string | query | 否 |  |
| eventType | string | query | 否 |  |
| type | string | query | 否 |  |
| domain | string | query | 否 |  |
| attemptCountGT | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetPendingWebhookEvents 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (選用)
	externalId := "externalId_example" // string |  (選用)
	eventType := "eventType_example" // string |  (選用)
	type_ := "type__example" // string |  (選用)
	domain := "domain_example" // string |  (選用)
	attemptCountGT := float64(1.2) // float64 |  (選用)
	skip := float64(1.2) // float64 |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPendingWebhookEvents`: GetPendingWebhookEvents200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]