Frühere Kommentatoren auf der Seite, die derzeit NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies, nachdem Sie /users/online erschöpft haben, um einen "Mitglieder"-Abschnitt anzuzeigen.
Cursor-Paginierung auf commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index von afterName nach vorne mittels $gt, ohne $skip-Kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Response

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Example

[inline-code-attrs-start title = 'GetOfflineUsers Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Seiten-URL-Kennung (serverseitig bereinigt).
	afterName := "afterName_example" // string | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. (optional)
	afterUserId := "afterUserId_example" // string | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei gleichen Namen nicht verloren gehen. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]