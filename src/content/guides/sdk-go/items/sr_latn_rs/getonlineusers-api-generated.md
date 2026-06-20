Trenutno online gledaoci stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici u okviru sobe, uključujući anonimne posetioce koje ne nabrajamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrešavanje izjednačenja: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se unosi sa istim imenom ne bi izostavili. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identifikator URL stranice (očišćen na serverskoj strani).
	afterName := "afterName_example" // string | Kursor: pošaljite nextAfterName iz prethodnog odgovora. (opciono)
	afterUserId := "afterUserId_example" // string | Kursor za razrešavanje izjednačenja: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se unosi sa istim imenom ne bi izostavili. (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]