## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |
| isFlagged | boolean | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de FlagCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isFlagged := true // bool | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.FlagCommentPublic(context.Background(), commentId).TenantId(tenantId).IsFlagged(isFlagged).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.FlagCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `FlagCommentPublic`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.FlagCommentPublic`: %v\n", resp)
}
[inline-code-end]