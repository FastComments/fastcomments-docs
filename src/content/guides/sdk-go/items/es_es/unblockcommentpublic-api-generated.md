---
## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_unblock_success.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de UnBlockCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	publicBlockFromCommentParams := *openapiclient.NewPublicBlockFromCommentParams([]string{"CommentIds_example"}) // PublicBlockFromCommentParams | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UnBlockCommentPublic(context.Background(), commentId).TenantId(tenantId).PublicBlockFromCommentParams(publicBlockFromCommentParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UnBlockCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `UnBlockCommentPublic`: UnblockSuccess
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UnBlockCommentPublic`: %v\n", resp)
}
[inline-code-end]

---