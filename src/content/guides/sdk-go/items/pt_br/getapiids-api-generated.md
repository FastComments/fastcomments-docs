## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Não |  |
| byIPFromComment | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| afterId | string | query | Não |  |
| demo | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comment_ids_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetApiIds'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (opcional)
	byIPFromComment := "byIPFromComment_example" // string |  (opcional)
	filters := "filters_example" // string |  (opcional)
	searchFilters := "searchFilters_example" // string |  (opcional)
	afterId := "afterId_example" // string |  (opcional)
	demo := true // bool |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiIds(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).AfterId(afterId).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetApiIds`: ModerationAPIGetCommentIdsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiIds`: %v\n", resp)
}
[inline-code-end]