## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_comment_pin_status_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo PinComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.PinComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.PinComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `PinComment`: ChangeCommentPinStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.PinComment`: %v\n", resp)
}
[inline-code-end]