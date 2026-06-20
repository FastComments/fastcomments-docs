Popis stranica za tenant. Koristi ga FChat desktop klijent za popunjavanje svog popisa soba.
Zahtijeva da je `enableFChat` postavljen na true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Veže se uz isti `sortBy`. |
| limit | integer | query | No | 1..200, zadano 50 |
| q | string | query | No | Opcionalni prefiks filtriranja naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice s najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Veže se uz isti `sortBy`. (neobavezno)
	limit := int32(56) // int32 | 1..200, zadano 50 (neobavezno)
	q := "q_example" // string | Opcionalni prefiks filtriranja naslova koji nije osjetljiv na velika/mala slova. (neobavezno)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (neobavezno)
	hasComments := true // bool | Ako je true, vraća samo stranice s najmanje jednim komentarom. (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]