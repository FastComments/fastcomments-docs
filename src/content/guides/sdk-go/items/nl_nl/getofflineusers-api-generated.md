Eerdere commentatoren op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat u /users/online hebt uitgeput om een "Leden"-sectie weer te geven.
Cursor-paginering op commenterName: de server loopt de partiële {tenantId, urlId, commenterName}
index vanaf afterName voorwaarts via $gt, geen $skip-kosten.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificatie (opgeschoond aan de serverzijde). |
| afterName | string | query | No | Cursor: geef nextAfterName door uit de vorige respons. |
| afterUserId | string | query | No | Cursor-tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items wegvallen. |

## Respons

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetOfflineUsers Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Pagina-URL-identificatie (opgeschoond aan de serverzijde).
	afterName := "afterName_example" // string | Cursor: geef nextAfterName door uit de vorige respons. (optioneel)
	afterUserId := "afterUserId_example" // string | Cursor tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items wegvallen. (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respons van `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]