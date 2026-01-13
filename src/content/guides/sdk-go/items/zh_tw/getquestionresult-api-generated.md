## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 回應

回傳: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_result_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetQuestionResult 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字串 | 
	id := "id_example" // 字串 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResult(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResult``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetQuestionResult` 的回應: GetQuestionResult200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResult`: %v\n", resp)
}
[inline-code-end]