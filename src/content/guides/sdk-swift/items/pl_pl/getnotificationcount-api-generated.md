## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| urlId | string | query | Nie |  |
| fromCommentId | string | query | Nie |  |
| viewed | boolean | query | Nie |  |
| type | string | query | Nie |  |

## Odpowiedź

Returns: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCountResponse.swift)

## Przykład

[inline-code-attrs-start title = 'getNotificationCount Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów, proszę zgłosić je poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcjonalnie)
let urlId = "urlId_example" // String |  (opcjonalnie)
let fromCommentId = "fromCommentId_example" // String |  (opcjonalnie)
let viewed = true // Bool |  (opcjonalnie)
let type = "type_example" // String |  (opcjonalnie)

DefaultAPI.getNotificationCount(tenantId: tenantId, options: DefaultAPI.GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---