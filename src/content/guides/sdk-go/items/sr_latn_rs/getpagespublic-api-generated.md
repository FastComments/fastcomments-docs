---
Lista stranica za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje njegove liste soba.
Zahteva da `enableFChat` bude true na razrešenom prilagođenom podešavanju za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se u skladu sa pristupom grupa korisnika koji šalje zahtev.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni kursor paginacije koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni prefiks filter naslova koji nije osetljiv na veličinu slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (alfabetički). |
| hasComments | boolean | query | No | Ako je true, vrati samo stranice sa najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Neprozirni kursor paginacije koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`. (optional)
	limit := int32(56) // int32 | 1..200, podrazumevano 50 (optional)
	q := "q_example" // string | Opcioni prefiks-filter naslova koji nije osetljiv na veličinu slova. (optional)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (alfabetički). (optional)
	hasComments := true // bool | Ako je true, vrati samo stranice sa najmanje jednim komentarom. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---