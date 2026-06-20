Attualmente spettatori online di una pagina: persone la cui sessione websocket è attualmente iscritta alla pagina. Restituisce anonCount + totalCount (iscritti alla stanza, inclusi gli spettatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore dell'URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non eliminino voci. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identificatore dell'URL della pagina (pulito lato server).
	afterName := "afterName_example" // string | Cursore: passare nextAfterName dalla risposta precedente. (opzionale)
	afterUserId := "afterUserId_example" // string | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non eliminino voci. (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]