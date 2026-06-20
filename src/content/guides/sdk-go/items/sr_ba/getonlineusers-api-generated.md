Trenutno online posjetioci stranice: osobe čija je websocket sesija trenutno pretplaćena na tu stranicu.
Vraća anonCount + totalCount (pretplatnici u sobi, uključujući anonimne gledaoce koje ne nabrajamo).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tajbrejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako pri istim imenima ne bi došlo do izostavljanja unosa. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identifikator URL-a stranice (očišćen na serverskoj strani).
	afterName := "afterName_example" // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
	afterUserId := "afterUserId_example" // string | Tajbrejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako pri istim imenima ne bi došlo do izostavljanja unosa. (opcionalno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]