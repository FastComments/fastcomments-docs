## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_suggest_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetSearchSuggest'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (opcjonalnie)
	sso := "sso_example" // string |  (opcjonalnie)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSuggest(context.Background()).TenantId(tenantId).TextSearch(textSearch).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Błąd podczas wywoływania `ModerationAPI.GetSearchSuggest``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Pełna odpowiedź HTTP: %v\n", r)
	}
	// odpowiedź z `GetSearchSuggest`: ModerationSuggestResponse
	fmt.Fprintf(os.Stdout, "Odpowiedź z `ModerationAPI.GetSearchSuggest`: %v\n", resp)
}
[inline-code-end]

---