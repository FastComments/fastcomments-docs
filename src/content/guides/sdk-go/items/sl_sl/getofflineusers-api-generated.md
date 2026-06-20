Pretekli komentatorji na strani, ki trenutno NISO online. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online za prikaz razdelka "Člani".
Stranjevanje z kurzorjem po commenterName: strežnik prehaja po parcialnem indeksu {tenantId, urlId, commenterName} od afterName naprej z uporabo $gt, brez stroška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (očiščen na strežniški strani). |
| afterName | string | query | Ne | Kursor: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kursor za razreševanje izenačenih imen: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se vnosi pri enakih imenih ne izgubijo. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

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
	urlId := "urlId_example" // string | Identifikator URL strani (očiščen na strežniški strani).
	afterName := "afterName_example" // string | Kursor: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
	afterUserId := "afterUserId_example" // string | Kursor za razreševanje izenačenih imen: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se vnosi pri enakih imenih ne izgubijo. (neobvezno)

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