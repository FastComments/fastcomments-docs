## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| urlId | string | query | Ne |  |
| fromCommentId | string | query | Ne |  |
| viewed | boolean | query | Ne |  |
| type | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršnokoli težavo prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (neobvezno)
let urlId = "urlId_example" // String |  (neobvezno)
let fromCommentId = "fromCommentId_example" // String |  (neobvezno)
let viewed = true // Bool |  (neobvezno)
let type = "type_example" // String |  (neobvezno)
let skip = 987 // Double |  (neobvezno)

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