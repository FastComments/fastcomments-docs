---
Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon iscrpljenja /users/online kako biste prikazali odjeljak "Članovi".
Paginacija kursorom po commenterName: server prolazi djelomični {tenantId, urlId, commenterName}
indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očisti se na strani poslužitelja). |
| afterName | string | query | Ne | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za rješavanje jednakih imena: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izostavljeni. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identifikator URL-a stranice (očisti se na strani poslužitelja).
	afterName := "afterName_example" // string | Kursor: pošaljite nextAfterName iz prethodnog odgovora. (opcionalno)
	afterUserId := "afterUserId_example" // string | Kursor za rješavanje jednakih imena: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izostavljeni. (opcionalno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---