Lijst met pagina's voor een tenant. Wordt gebruikt door de FChat-desktopclient om de kamerlijst te vullen.
Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen, worden gefilterd op basis van de groepstoegang van de aanvragende gebruiker.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Ondoorzichtige paginacursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | No | 1..200, standaard 50 |
| q | string | query | No | Optionele case-insensitieve prefixfilter voor titels. |
| sortBy | string | query | No | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | No | Als true, retourneer alleen pagina's met ten minste één opmerking. |

## Respons

Retourneert: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetPagesPublic Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Ondoorzichtige paginacursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. (optioneel)
	limit := int32(56) // int32 | 1..200, standaard 50 (optioneel)
	q := "q_example" // string | Optionele case-insensitieve prefixfilter voor titels. (optioneel)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). (optioneel)
	hasComments := true // bool | Als true, retourneer alleen pagina's met ten minste één opmerking. (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]