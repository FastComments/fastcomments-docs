## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| questionId | string | query | नहीं |  |
| questionIds | array | query | नहीं |  |
| urlId | string | query | नहीं |  |
| timeBucket | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| forceRecalculate | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_response.go)

## उदाहरण

[inline-code-attrs-start title = 'AggregateQuestionResults उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	questionId := "questionId_example" // string |  (वैकल्पिक)
	questionIds := []string{"Inner_example"} // []string |  (वैकल्पिक)
	urlId := "urlId_example" // string |  (वैकल्पिक)
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (वैकल्पिक)
	startDate := time.Now() // time.Time |  (वैकल्पिक)
	forceRecalculate := true // bool |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AggregateQuestionResults`: AggregateQuestionResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]