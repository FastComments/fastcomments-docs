## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Respuesta

Devuelve: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_sso_user_api_response.go)

## Ejemplo

[inline-code-attrs-start title = 'DeleteSSOUser Ejemplo'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	deleteComments := true // bool |  (opcional)
	commentDeleteMode := "commentDeleteMode_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSSOUser(context.Background(), id).TenantId(tenantId).DeleteComments(deleteComments).CommentDeleteMode(commentDeleteMode).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `DeleteSSOUser`: DeleteSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSSOUser`: %v\n", resp)
}
[inline-code-end]

---