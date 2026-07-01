List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niewidoczny wskaźnik paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu nie uwzględniający wielkości liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony zawierające przynajmniej jeden komentarz. |

## Response

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów prosimy zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Niewidoczny wskaźnik paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. (optional)
let limit = 987 // Int | 1..200, domyślnie 50 (optional)
let q = "q_example" // String | Opcjonalny filtr prefiksu tytułu nie uwzględniający wielkości liter. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie). (optional)
let hasComments = true // Bool | Jeśli true, zwróć tylko strony zawierające przynajmniej jeden komentarz. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]