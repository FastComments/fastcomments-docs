## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| urlId | string | query | Nie |  |
| fromCommentId | string | query | Nie |  |
| viewed | boolean | query | Nie |  |
| type | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcjonalne)
let urlId = "urlId_example" // String |  (opcjonalne)
let fromCommentId = "fromCommentId_example" // String |  (opcjonalne)
let viewed = true // Bool |  (opcjonalne)
let type = "type_example" // String |  (opcjonalne)
let skip = 987 // Double |  (opcjonalne)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]