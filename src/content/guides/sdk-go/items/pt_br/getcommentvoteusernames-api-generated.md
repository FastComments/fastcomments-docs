## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| dir | integer | query | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_vote_user_names_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetCommentVoteUserNames'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	dir := int32(56) // int32 | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentVoteUserNames(context.Background(), tenantId, commentId).Dir(dir).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentVoteUserNames``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetCommentVoteUserNames`: GetCommentVoteUserNames200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentVoteUserNames`: %v\n", resp)
}
[inline-code-end]

---