Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da biste prikazali sekciju "Members".
Kursor-paginacija po commenterName: server prolazi delimični {tenantId, urlId, commenterName}
indeks od afterName unapred preko $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi sa istim imenom ne bi bili izostavljeni. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterName := "afterName_example" // string | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
	afterUserId := "afterUserId_example" // string | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi sa istim imenom ne bi bili izostavljeni. (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]