## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| locale | string | query | Ne |  |
| rating | string | query | Ne |  |
| page | number | query | Ne |  |

## Odgovor

Vrne: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_trending_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetGifsTrending'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (neobvezno)
	rating := "rating_example" // string |  (neobvezno)
	page := float64(1.2) // float64 |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsTrending(context.Background(), tenantId).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsTrending``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetGifsTrending`: GetGifsTrendingResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsTrending`: %v\n", resp)
}
[inline-code-end]