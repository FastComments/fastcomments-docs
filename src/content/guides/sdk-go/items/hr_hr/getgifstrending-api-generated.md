---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| locale | string | upit | Ne |  |
| rating | string | upit | Ne |  |
| page | number | upit | Ne |  |

## Odgovor

Vraća: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_trending_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetGifsTrending'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (neobavezno)
	rating := "rating_example" // string |  (neobavezno)
	page := float64(1.2) // float64 |  (neobavezno)

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

---