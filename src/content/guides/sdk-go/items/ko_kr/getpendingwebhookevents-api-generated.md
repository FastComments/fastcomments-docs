## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니요 |  |
| externalId | string | query | 아니요 |  |
| eventType | string | query | 아니요 |  |
| type | string | query | 아니요 |  |
| domain | string | query | 아니요 |  |
| attemptCountGT | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pending_webhook_events_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetPendingWebhookEvents 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |  (선택 사항)
	externalId := "externalId_example" // string |  (선택 사항)
	eventType := "eventType_example" // string |  (선택 사항)
	type_ := "type__example" // string |  (선택 사항)
	domain := "domain_example" // string |  (선택 사항)
	attemptCountGT := float64(1.2) // float64 |  (선택 사항)
	skip := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPendingWebhookEvents(context.Background()).TenantId(tenantId).CommentId(commentId).ExternalId(externalId).EventType(eventType).Type_(type_).Domain(domain).AttemptCountGT(attemptCountGT).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPendingWebhookEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPendingWebhookEvents`의 응답: GetPendingWebhookEvents200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPendingWebhookEvents`: %v\n", resp)
}
[inline-code-end]

---