## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo PostSetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	setCommentTextParams := *openapiclient.NewSetCommentTextParams("Comment_example") // SetCommentTextParams | 
	broadcastId := "broadcastId_example" // string |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentText(context.Background(), commentId).TenantId(tenantId).SetCommentTextParams(setCommentTextParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erro ao chamar `ModerationAPI.PostSetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Resposta HTTP completa: %v\n", r)
	}
	// resposta de `PostSetCommentText`: SetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Resposta de `ModerationAPI.PostSetCommentText`: %v\n", resp)
}
[inline-code-end]