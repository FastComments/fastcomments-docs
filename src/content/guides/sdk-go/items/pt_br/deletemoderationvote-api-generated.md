## Parâmetros

| Nome | Tipo | Location | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| voteId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de DeleteModerationVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	voteId := "voteId_example" // string | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.DeleteModerationVote(context.Background(), commentId, voteId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.DeleteModerationVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `DeleteModerationVote`: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.DeleteModerationVote`: %v\n", resp)
}
[inline-code-end]