---
## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| search | string | query | Sim |  |
| locale | string | query | Não |  |
| rating | string | query | Não |  |
| page | number | query | Não |  |

## Resposta

Retorna: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_search_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetGifsSearch'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	search := "search_example" // string | 
	locale := "locale_example" // string |  (opcional)
	rating := "rating_example" // string |  (opcional)
	page := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsSearch(context.Background(), tenantId).Search(search).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsSearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetGifsSearch`: GetGifsSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsSearch`: %v\n", resp)
}
[inline-code-end]

---