## Parámetros

| Name | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de UpdateComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	body := PickAPICommentUpdatableCommentFields(987) // PickAPICommentUpdatableCommentFields | 
	contextUserId := "contextUserId_example" // string |  (opcional)
	doSpamCheck := true // bool |  (opcional)
	isLive := true // bool |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateComment(context.Background(), id).TenantId(tenantId).Body(body).ContextUserId(contextUserId).DoSpamCheck(doSpamCheck).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `UpdateComment`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateComment`: %v\n", resp)
}
[inline-code-end]