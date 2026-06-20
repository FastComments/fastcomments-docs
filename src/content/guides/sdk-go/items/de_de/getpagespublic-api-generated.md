Listet Seiten für einen Mandanten. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen. Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist. Seiten, die SSO erfordern, werden anhand des Gruppenzugriffs des anfragenden Benutzers gefiltert.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Undurchsichtiger Paginierungs-Cursor, zurückgegeben als `nextCursor` aus einer vorherigen Anfrage. An denselben `sortBy` gebunden. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, case-insensitiver Filter für Titelpräfix. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Undurchsichtiger Paginierungs-Cursor, zurückgegeben als `nextCursor` aus einer vorherigen Anfrage. An denselben `sortBy` gebunden. (optional)
	limit := int32(56) // int32 | 1..200, Standard 50 (optional)
	q := "q_example" // string | Optionaler, case-insensitiver Filter für Titelpräfix. (optional)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). (optional)
	hasComments := true // bool | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben. (optional)

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