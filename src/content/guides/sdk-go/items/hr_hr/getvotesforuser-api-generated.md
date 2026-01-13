## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_for_user_200_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetVotesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (nije obavezno)
	anonUserId := "anonUserId_example" // string |  (nije obavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotesForUser(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetVotesForUser`: GetVotesForUser200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotesForUser`: %v\n", resp)
}
[inline-code-end]