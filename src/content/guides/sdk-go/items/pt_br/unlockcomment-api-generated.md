## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`LockComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_lock_comment_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de UnLockComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UnLockComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UnLockComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `UnLockComment`: LockComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UnLockComment`: %v\n", resp)
}
[inline-code-end]