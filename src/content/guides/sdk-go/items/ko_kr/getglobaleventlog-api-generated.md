req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| urlId | string | 쿼리 | 예 |  |
| userIdWS | string | 쿼리 | 예 |  |
| startTime | integer | 쿼리 | 예 |  |
| endTime | integer | 쿼리 | 아니요 |  |

## 응답

반환: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_response.go)

## 예제

[inline-code-attrs-start title = 'GetGlobalEventLog 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGlobalEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGlobalEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetGlobalEventLog`의 응답: GetEventLogResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGlobalEventLog`: %v\n", resp)
}
[inline-code-end]