## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Sim |  |
| editKey | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de SetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string | 
	commentTextUpdateRequest := *openapiclient.NewCommentTextUpdateRequest("Comment_example") // CommentTextUpdateRequest | 
	editKey := "editKey_example" // string |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SetCommentText(context.Background(), tenantId, commentId).BroadcastId(broadcastId).CommentTextUpdateRequest(commentTextUpdateRequest).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `SetCommentText`: SetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SetCommentText`: %v\n", resp)
}
[inline-code-end]