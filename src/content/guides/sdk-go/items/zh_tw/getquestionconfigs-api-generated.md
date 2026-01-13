## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_configs_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetQuestionConfigs 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionConfigs(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionConfigs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetQuestionConfigs` 的回應: GetQuestionConfigs200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionConfigs`: %v\n", resp)
}
[inline-code-end]

---