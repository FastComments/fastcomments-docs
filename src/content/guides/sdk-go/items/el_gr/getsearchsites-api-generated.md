## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| value | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_site_search_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetSearchSites'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSites(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchSites``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetSearchSites`: ModerationSiteSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchSites`: %v\n", resp)
}
[inline-code-end]

---