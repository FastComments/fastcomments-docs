## 参数

| 名称 | 类型 | 位置 | 是否必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回：[`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_question_result_200_response.go)

## 示例

[inline-code-attrs-start title = 'CreateQuestionResult 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createQuestionResultBody := *openapiclient.NewCreateQuestionResultBody("UrlId_example", float64(123), "QuestionId_example") // CreateQuestionResultBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateQuestionResult(context.Background()).TenantId(tenantId).CreateQuestionResultBody(createQuestionResultBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateQuestionResult``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateQuestionResult`: CreateQuestionResult200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateQuestionResult`: %v\n", resp)
}
[inline-code-end]

---