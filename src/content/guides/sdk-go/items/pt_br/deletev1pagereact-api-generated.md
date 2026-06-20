## Parâmetros

| Nome | Tipo | Location | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |

## Resposta

Retorna: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de DeleteV1PageReact'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteV1PageReact(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `DeleteV1PageReact`: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteV1PageReact`: %v\n", resp)
}
[inline-code-end]