---
Commentatori passati sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usare questo dopo aver esaurito /users/online per visualizzare una sezione "Membri".
Paginazione cursore su commenterName: il server scorre l'indice parziale {tenantId, urlId, commenterName}
dalla afterName in avanti tramite $gt, senza costo $skip.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Criterio di spareggio del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato per evitare che le voci con lo stesso nome vengano scartate. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identificatore URL della pagina (ripulito lato server).
	afterName := "afterName_example" // string | Cursore: passare nextAfterName dalla risposta precedente. (opzionale)
	afterUserId := "afterUserId_example" // string | Criterio di spareggio del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato per evitare che le voci con lo stesso nome vengano scartate. (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---