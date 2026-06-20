## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| urlId | string | query | Ne |  |
| fromCommentId | string | query | Ne |  |
| viewed | boolean | query | Ne |  |
| type | string | query | Ne |  |

## Odgovor

Vrne: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCountResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getNotificationCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za kakršno koli težavo jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (izbirno)
let urlId = "urlId_example" // String |  (izbirno)
let fromCommentId = "fromCommentId_example" // String |  (izbirno)
let viewed = true // Bool |  (izbirno)
let type = "type_example" // String |  (izbirno)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]