## Parámetros

| Name | Type | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| commentId | string | ruta | Sí |  |
| voteId | string | ruta | Sí |  |
| urlId | string | consulta | Sí |  |
| broadcastId | string | consulta | Sí |  |
| editKey | string | consulta | No |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de DeleteCommentVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	voteId := "voteId_example" // string | 
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	editKey := "editKey_example" // string |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteCommentVote(context.Background(), tenantId, commentId, voteId).UrlId(urlId).BroadcastId(broadcastId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteCommentVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `DeleteCommentVote`: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteCommentVote`: %v\n", resp)
}
[inline-code-end]