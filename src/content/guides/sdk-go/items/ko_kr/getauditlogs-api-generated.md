## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |
| order | string | query | 아니오 |  |
| after | number | query | 아니오 |  |
| before | number | query | 아니오 |  |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_response.go)

## 예제

[inline-code-attrs-start title = 'GetAuditLogs 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (선택 사항)
	skip := float64(1.2) // float64 |  (선택 사항)
	order := openapiclient.SORT_DIR("ASC") // SORTDIR |  (선택 사항)
	after := float64(1.2) // float64 |  (선택 사항)
	before := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAuditLogs(context.Background()).TenantId(tenantId).Limit(limit).Skip(skip).Order(order).After(after).Before(before).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAuditLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetAuditLogs`의 응답: GetAuditLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]