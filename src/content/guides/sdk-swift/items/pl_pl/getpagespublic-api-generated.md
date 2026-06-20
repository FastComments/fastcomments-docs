Lista stron dla najemcy. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązywanym niestandardowym configu dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika żądającego.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny prefiks tytułu niewrażliwy na wielkość liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najwięcej komentarzy najpierw), lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów zgłaszaj poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z wcześniejszego żądania. Powiązany z tym samym `sortBy`. (opcjonalne)
let limit = 987 // Int | 1..200, domyślnie 50 (opcjonalne)
let q = "q_example" // String | Opcjonalny, niewrażliwy na wielkość liter filtr prefiksu tytułu. (opcjonalne)
let sortBy = PagesSortBy() // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najwięcej komentarzy najpierw), lub `title` (alfabetycznie). (opcjonalne)
let hasComments = true // Bool | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. (opcjonalne)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]