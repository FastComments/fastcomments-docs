Trenutno prisotni gledalci strani: osebe, katerih websocket seja je trenutno naročena na stran. Vrne anonCount + totalCount (naročniki v sobi, vključno z anonimnimi gledalci, ki jih ne naštevamo).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (očiščeno na strežniku). |
| afterName | string | query | Ne | Kursor: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Rešitelj izenačenj v kursorju: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, kadar je afterName nastavljeno, da se vnosi ob izenačenih imen ne izgubijo. |

## Odgovor

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

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
	urlId := "urlId_example" // string | Identifikator URL strani (očiščeno na strežniku).
	afterName := "afterName_example" // string | Kursor: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
	afterUserId := "afterUserId_example" // string | Rešitelj izenačenj v kursorju: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, kadar je afterName nastavljeno, da se vnosi ob izenačenih imen ne izgubijo. (neobvezno)

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