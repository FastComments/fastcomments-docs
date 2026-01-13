## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| startDate | string | query | לא |  |
| questionId | string | query | לא |  |
| questionIds | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת GetQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (אופציונלי)
	userId := "userId_example" // string |  (אופציונלי)
	startDate := "startDate_example" // string |  (אופציונלי)
	questionId := "questionId_example" // string |  (אופציונלי)
	questionIds := "questionIds_example" // string |  (אופציונלי)
	skip := float64(1.2) // float64 |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetQuestionResults`: GetQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]

---