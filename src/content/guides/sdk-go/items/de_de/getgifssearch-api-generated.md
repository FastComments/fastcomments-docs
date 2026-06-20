## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| search | string | query | Ja |  |
| locale | string | query | Nein |  |
| rating | string | query | Nein |  |
| page | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_search_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetGifsSearch Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (optional)
	rating := "rating_example" // string |  (optional)
	page := float64(1.2) // float64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsSearch(context.Background(), tenantId).Search(search).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsSearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetGifsSearch`: GetGifsSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsSearch`: %v\n", resp)
}
[inline-code-end]