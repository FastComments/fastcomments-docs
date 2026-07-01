## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| urlId | string | query | Ne |  |
| fromCommentId | string | query | Ne |  |
| viewed | boolean | query | Ne |  |
| type | string | query | Ne |  |

## Response

Vrne: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCountResponse.swift)

## Example

[inline-code-attrs-start title = 'getNotificationCount Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za morebitne težave jih prosimo sporočite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (neobvezno)
let urlId = "urlId_example" // String |  (neobvezno)
let fromCommentId = "fromCommentId_example" // String |  (neobvezno)
let viewed = true // Bool |  (neobvezno)
let type = "type_example" // String |  (neobvezno)

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