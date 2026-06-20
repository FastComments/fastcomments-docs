Massenhafte Benutzerinformationen für einen Mandanten. Bei gegebenen userIds werden Anzeigeinformationen aus User / SSOUser zurückgegeben.
Wird vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade durch ein Presence-Ereignis aufgetaucht sind.
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Pfad | Ja |  |
| ids | string | Abfrage | Ja | Kommagetrennte userIds. |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetUsersInfo'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | Comma-delimited userIds.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]

---