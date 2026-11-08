## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| search | string | query | हाँ |  |
| locale | string | query | नहीं |  |
| rating | string | query | नहीं |  |
| page | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_search_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetGifsSearch उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (वैकल्पिक)
	rating := "rating_example" // string |  (वैकल्पिक)
	page := float64(1.2) // float64 |  (वैकल्पिक)

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