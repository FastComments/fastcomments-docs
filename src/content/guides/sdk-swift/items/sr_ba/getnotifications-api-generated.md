## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| urlId | string | query | Ne |  |
| fromCommentId | string | query | Ne |  |
| viewed | boolean | query | Ne |  |
| type | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let userId = "userId_example" // String |  (opcionalno)
let urlId = "urlId_example" // String |  (opcionalno)
let fromCommentId = "fromCommentId_example" // String |  (opcionalno)
let viewed = true // Bool |  (opcionalno)
let type = "type_example" // String |  (opcionalno)
let skip = 987 // Double |  (opcionalno)

DefaultAPI.getNotifications(tenantId: tenantId, options: DefaultAPI.GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip)) { (response, error) in
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