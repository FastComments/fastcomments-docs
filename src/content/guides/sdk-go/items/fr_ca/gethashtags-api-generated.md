## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| page | number | query | Non |  |

## Réponse

Retourne : [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_hash_tags_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetHashTags'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetHashTags(context.Background()).TenantId(tenantId).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetHashTags``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetHashTags`: GetHashTags200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetHashTags`: %v\n", resp)
}
[inline-code-end]