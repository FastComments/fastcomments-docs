## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_moderators_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetModerators 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetModerators(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetModerators``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetModerators`의 응답: GetModerators200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetModerators`: %v\n", resp)
}
[inline-code-end]