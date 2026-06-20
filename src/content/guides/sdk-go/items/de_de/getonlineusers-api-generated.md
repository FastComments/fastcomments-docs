Aktuell online befindliche Betrachter einer Seite: Personen, deren WebSocket-Sitzung momentan auf die Seite abonniert ist.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht einzeln auflisten).

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Bezeichner (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. |

## Response

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Example

[inline-code-attrs-start title = 'GetOnlineUsers Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Seiten-URL-Bezeichner (serverseitig bereinigt).
	afterName := "afterName_example" // string | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. (optional)
	afterUserId := "afterUserId_example" // string | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]