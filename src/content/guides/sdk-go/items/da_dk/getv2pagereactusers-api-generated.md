## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| id | string | query | Ja |  |

## Svar

Returnerer: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v2_page_react_users_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetV2PageReactUsers Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetV2PageReactUsers(context.Background(), tenantId).UrlId(urlId).Id(id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV2PageReactUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetV2PageReactUsers`: GetV2PageReactUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV2PageReactUsers`: %v\n", resp)
}
[inline-code-end]