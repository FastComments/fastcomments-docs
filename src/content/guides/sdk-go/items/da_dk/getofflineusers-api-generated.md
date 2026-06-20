---
Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en 'Medlemmer'-sektion.
Cursor-paginering på commenterName: serveren går igennem det delvise {tenantId, urlId, commenterName}-index fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | Nej | Cursor: angiv nextAfterName fra det forrige svar. (valgfri) |
| afterUserId | string | query | Nej | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navnekollisioner ikke udelader poster. (valgfri) |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterUserId := "afterUserId_example" // string | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navnekollisioner ikke udelader poster. (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---