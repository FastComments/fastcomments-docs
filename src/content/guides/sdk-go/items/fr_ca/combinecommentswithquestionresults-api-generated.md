---
## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| questionId | string | query | Non |  |
| questionIds | array | query | Non |  |
| urlId | string | query | Non |  |
| startDate | string | query | Non |  |
| forceRecalculate | boolean | query | Non |  |
| minValue | number | query | Non |  |
| maxValue | number | query | Non |  |
| limit | number | query | Non |  |

## Réponse

Retourne: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	questionId := "questionId_example" // string |  (optionnel)
	questionIds := []string{"Inner_example"} // []string |  (optionnel)
	urlId := "urlId_example" // string |  (optionnel)
	startDate := time.Now() // time.Time |  (optionnel)
	forceRecalculate := true // bool |  (optionnel)
	minValue := float64(1.2) // float64 |  (optionnel)
	maxValue := float64(1.2) // float64 |  (optionnel)
	limit := float64(1.2) // float64 |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]

---