## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in fase beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opzionale)
let urlId = "urlId_example" // String |  (opzionale)
let fromCommentId = "fromCommentId_example" // String |  (opzionale)
let viewed = true // Bool |  (opzionale)
let type = "type_example" // String |  (opzionale)
let skip = 987 // Double |  (opzionale)

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

---