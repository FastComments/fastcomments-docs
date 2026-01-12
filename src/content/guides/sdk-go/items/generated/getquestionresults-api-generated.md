## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Response

Returns: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_200_response.go)

## Example

[inline-code-attrs-start title = 'GetQuestionResults Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (optional)
	userId := "userId_example" // string |  (optional)
	startDate := "startDate_example" // string |  (optional)
	questionId := "questionId_example" // string |  (optional)
	questionIds := "questionIds_example" // string |  (optional)
	skip := float64(1.2) // float64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetQuestionResults`: GetQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]
