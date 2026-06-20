## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| search | string | query | Sì |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Risposta

Restituisce: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_search_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio GetGifsSearch'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (opzionale)
	rating := "rating_example" // string |  (opzionale)
	page := float64(1.2) // float64 |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsSearch(context.Background(), tenantId).Search(search).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsSearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetGifsSearch`: GetGifsSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsSearch`: %v\n", resp)
}
[inline-code-end]