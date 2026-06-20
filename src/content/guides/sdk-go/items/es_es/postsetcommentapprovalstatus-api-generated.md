## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| approved | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_approved_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de PostSetCommentApprovalStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	approved := true // bool |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentApprovalStatus(context.Background(), commentId).Approved(approved).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentApprovalStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `PostSetCommentApprovalStatus`: SetCommentApprovedResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentApprovalStatus`: %v\n", resp)
}
[inline-code-end]