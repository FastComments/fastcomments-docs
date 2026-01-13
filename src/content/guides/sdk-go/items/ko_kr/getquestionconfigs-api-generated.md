## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| skip | number | 쿼리 | 아니요 |  |

## 응답

반환: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_configs_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetQuestionConfigs 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetQuestionConfigs(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionConfigs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetQuestionConfigs`의 응답: GetQuestionConfigs200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionConfigs`: %v\n", resp)
}
[inline-code-end]