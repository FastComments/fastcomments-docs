Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista di stanze. Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina. Le pagine che richiedono SSO sono filtrate in base all'accesso del gruppo dell'utente richiedente.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale sul prefisso del titolo, insensibile alle maiuscole. |
| sortBy | string | query | No | Criterio di ordinamento. `updatedAt` (predefinito, più recenti prima), `commentCount` (più commenti prima), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. (opzionale)
	limit := int32(56) // int32 | 1..200, predefinito 50 (opzionale)
	q := "q_example" // string | Filtro opzionale sul prefisso del titolo, insensibile alle maiuscole. (opzionale)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Criterio di ordinamento. `updatedAt` (predefinito, più recenti prima), `commentCount` (più commenti prima), o `title` (alfabetico). (opzionale)
	hasComments := true // bool | Se true, restituisce solo le pagine con almeno un commento. (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---