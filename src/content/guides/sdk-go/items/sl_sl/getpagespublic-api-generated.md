---
Seznam strani za najemnika. Uporablja ga namizni odjemalec FChat za polnjenje svojega seznama sob.
Zahteva, da je `enableFChat` v razrešeni prilagojeni konfiguraciji za vsako stran nastavljen na true.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop skupin uporabnika, ki pošilja zahtevo.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kurzor paginacije, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | Ne | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejši prvi), `commentCount` (največ komentarjev prvi), ali `title` (po abecedi). |
| hasComments | boolean | query | Ne | Če je true, vrne le strani z vsaj enim komentarjem. |

## Odgovor

Vrača: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Primer

[inline-code-attrs-start title = 'GetPagesPublic Primer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Neprozoren kurzor paginacije, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`. (neobvezno)
	limit := int32(56) // int32 | 1..200, privzeto 50 (neobvezno)
	q := "q_example" // string | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. (neobvezno)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejši prvi), `commentCount` (največ komentarjev prvi) ali `title` (po abecedi). (neobvezno)
	hasComments := true // bool | Če je true, vrne le strani z vsaj enim komentarjem. (neobvezno)

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