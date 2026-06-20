## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| locale | string | query | Nie |  |
| rating | string | query | Nie |  |
| page | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_trending_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetGifsTrending'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (opcjonalne)
	rating := "rating_example" // string |  (opcjonalne)
	page := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsTrending(context.Background(), tenantId).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsTrending``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetGifsTrending`: GetGifsTrendingResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsTrending`: %v\n", resp)
}
[inline-code-end]