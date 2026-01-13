## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |

## Réponse

Renvoie: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple pour GetVotes'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotes(context.Background()).TenantId(tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetVotes`: GetVotes200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotes`: %v\n", resp)
}
[inline-code-end]