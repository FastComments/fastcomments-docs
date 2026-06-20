Lister sider for en tenant. Bruges af FChat-desktopklienten til at udfylde dens rumliste.
Kræver, at `enableFChat` er true i den opløste custom config for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nej | Uigennemsigtig pagineringscursor, der returneres som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. |
| limit | integer | query | Nej | 1..200, standard 50 |
| q | string | query | Nej | Valgfrit case-insensitivt titelpræfiksfilter. |
| sortBy | string | query | Nej | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | Nej | Hvis true, returner kun sider med mindst én kommentar. |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetPagesPublic Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Uigennemsigtig pagineringscursor, der returneres som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. (valgfri)
	limit := int32(56) // int32 | 1..200, standard 50 (valgfri)
	q := "q_example" // string | Valgfrit case-insensitivt titelpræfiksfilter. (valgfri)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). (valgfri)
	hasComments := true // bool | Hvis true, returner kun sider med mindst én kommentar. (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]