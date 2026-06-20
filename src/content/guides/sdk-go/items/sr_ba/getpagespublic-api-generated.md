---
Prikazuje stranice za tenant. Koristi ga FChat desktop klijent da popuni svoju listu soba.
Zahtijeva `enableFChat` da bude true na riješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Opaque kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Opcionalni filter po prefiksu naslova neosjetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prve), `commentCount` (stranice s najviše komentara prve), ili `title` (alfabetski). |
| hasComments | boolean | query | Ne | Ako je true, vrati samo stranice koje imaju bar jedan komentar. |

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
	cursor := "cursor_example" // string | Opaque kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`. (opcionalno)
	limit := int32(56) // int32 | 1..200, zadano 50 (opcionalno)
	q := "q_example" // string | Opcionalni filter po prefiksu naslova neosjetljiv na velika i mala slova. (opcionalno)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prve), `commentCount` (stranice s najviše komentara prve), ili `title` (alfabetski). (opcionalno)
	hasComments := true // bool | Ako je true, vrati samo stranice koje imaju bar jedan komentar. (opcionalno)

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

---