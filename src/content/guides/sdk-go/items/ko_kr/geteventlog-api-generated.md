req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| userIdWS | string | query | 예 |  |
| startTime | integer | query | 예 |  |
| endTime | integer | query | 예 |  |

## 응답

반환: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetEventLog 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetEventLog`의 응답: GetEventLog200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetEventLog`: %v\n", resp)
}
[inline-code-end]

---