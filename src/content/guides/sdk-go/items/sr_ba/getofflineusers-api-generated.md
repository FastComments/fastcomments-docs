Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da prikažete sekciju "Members".
Kursor paginacija po commenterName: server koristi djelimični {tenantId, urlId, commenterName}
indeks od afterName unaprijed putem $gt, bez troška $skip.

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (obrađen na serverskoj strani). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tajm-brejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi sa istim imenom ne bi bili izostavljeni. |

## Response

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
	urlId := "urlId_example" // string | Identifikator URL stranice (obrađen na serverskoj strani).
	afterName := "afterName_example" // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opciono)
	afterUserId := "afterUserId_example" // string | Tajm-brejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi sa istim imenom ne bi bili izostavljeni. (opciono)

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