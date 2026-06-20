## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_hash_tags_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetHashTags'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 | (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetHashTags(context.Background()).TenantId(tenantId).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetHashTags``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetHashTags`: GetHashTagsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetHashTags`: %v\n", resp)
}
[inline-code-end]

---