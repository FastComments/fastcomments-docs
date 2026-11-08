## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionIds | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetQuestionResults उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (वैकल्पिक)
	userId := "userId_example" // string |  (वैकल्पिक)
	startDate := "startDate_example" // string |  (वैकल्पिक)
	questionId := "questionId_example" // string |  (वैकल्पिक)
	questionIds := "questionIds_example" // string |  (वैकल्पिक)
	skip := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetQuestionResults` से प्रतिक्रिया: GetQuestionResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]