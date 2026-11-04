Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga ustawienia `enableFChat` na true w rozwiązanej konfiguracji niestandardowej dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| cursor | string | query | Nie | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | Nie | 1..200, domyślnie 50 |
| q | string | query | Nie | Opcjonalny filtr prefiksu tytułu, niewrażliwy na wielkość liter. |
| sortBy | string | query | Nie | Kolejność sortowania. `updatedAt` (domyślna, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie). |
| hasComments | boolean | query | Nie | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. (optional)
let limit = 987 // Int | 1..200, domyślnie 50 (optional)
let q = "q_example" // String | Opcjonalny filtr prefiksu tytułu, niewrażliwy na wielkość liter. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślna, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie). (optional)
let hasComments = true // Bool | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem. (optional)

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