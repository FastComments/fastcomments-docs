## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_question_config_response.go)

## 例

[inline-code-attrs-start title = 'CreateQuestionConfig の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createQuestionConfigBody := *openapiclient.NewCreateQuestionConfigBody("Name_example", "Question_example", "Type_example", float64(123)) // CreateQuestionConfigBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateQuestionConfig(context.Background()).TenantId(tenantId).CreateQuestionConfigBody(createQuestionConfigBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateQuestionConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateQuestionConfig` からのレスポンス: CreateQuestionConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateQuestionConfig`: %v\n", resp)
}
[inline-code-end]

---