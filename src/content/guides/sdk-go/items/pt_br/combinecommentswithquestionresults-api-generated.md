## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| questionId | string | query | Não |  |
| questionIds | array | query | Não |  |
| urlId | string | query | Não |  |
| startDate | string | query | Não |  |
| forceRecalculate | boolean | query | Não |  |
| minValue | number | query | Não |  |
| maxValue | number | query | Não |  |
| limit | number | query | Não |  |

## Resposta

Retorna: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (opcional)
	questionIds := []string{"Inner_example"} // []string |  (opcional)
	urlId := "urlId_example" // string |  (opcional)
	startDate := time.Now() // time.Time |  (opcional)
	forceRecalculate := true // bool |  (opcional)
	minValue := float64(1.2) // float64 |  (opcional)
	maxValue := float64(1.2) // float64 |  (opcional)
	limit := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]

---