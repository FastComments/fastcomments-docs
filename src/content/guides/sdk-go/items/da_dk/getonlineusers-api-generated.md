Aktuelt-online seere af en side: personer hvis websocket-session er tilmeldt siden lige nu.
Returnerer anonCount + totalCount (abonnenter på tværs af rummet, inklusive anonyme seere, som vi ikke opregner).

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | sti | Ja |  |
| urlId | string | forespørgsel | Ja | Side-URL-identifikator (renset på serversiden). |
| afterName | string | forespørgsel | Nej | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | forespørgsel | Nej | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så poster med samme navn ikke udelades. |

## Svar

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetOnlineUsers Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Side-URL-identifikator (renset på serversiden).
	afterName := "afterName_example" // string | Cursor: angiv nextAfterName fra det forrige svar. (valgfri)
	afterUserId := "afterUserId_example" // string | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet, når afterName er sat, så poster med samme navn ikke udelades. (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]