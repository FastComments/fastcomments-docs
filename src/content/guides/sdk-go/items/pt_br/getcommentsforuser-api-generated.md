## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| direction | string | query | Não |  |
| repliesToUserId | string | query | Não |  |
| page | number | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| isCrawler | boolean | query | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetCommentsForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (opcional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcional)
	repliesToUserId := "repliesToUserId_example" // string |  (opcional)
	page := float64(1.2) // float64 |  (opcional)
	includei10n := true // bool |  (opcional)
	locale := "locale_example" // string |  (opcional)
	isCrawler := true // bool |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetCommentsForUser`: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]

---