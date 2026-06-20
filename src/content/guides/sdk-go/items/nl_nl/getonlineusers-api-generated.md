Momenteel online kijkers van een pagina: mensen wiens websocket-sessie op dit moment op de pagina is geabonneerd.
Retourneert anonCount + totalCount (room-brede abonnees, inclusief anonieme kijkers die we niet afzonderlijk opsommen).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificatie (op de server opgeschoond). |
| afterName | string | query | No | Cursor: geef nextAfterName door uit het vorige antwoord. |
| afterUserId | string | query | No | Tiebreaker voor cursor: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen vermeldingen wegvallen. |

## Response

Retourneert: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetOnlineUsers Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Pagina-URL-identificatie (op de server opgeschoond).
	afterName := "afterName_example" // string | Cursor: geef nextAfterName door uit het vorige antwoord. (optioneel)
	afterUserId := "afterUserId_example" // string | Tiebreaker voor cursor: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen vermeldingen wegvallen. (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]