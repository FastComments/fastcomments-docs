## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | ruta | Sí |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de PostSetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	setCommentTextParams := *openapiclient.NewSetCommentTextParams("Comment_example") // SetCommentTextParams | 
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentText(context.Background(), commentId).SetCommentTextParams(setCommentTextParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `PostSetCommentText`: SetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentText`: %v\n", resp)
}
[inline-code-end]