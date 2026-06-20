Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergavegegevens van User / SSOUser.
Wordt gebruikt door de commentaar-widget om gebruikers aan te vullen die zojuist verschenen via een presence-evenement.
Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Door komma's gescheiden userIds. |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetUsersInfo Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | Door komma's gescheiden userIds.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Reactie van `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]

---