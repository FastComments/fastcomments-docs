---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| urlId | string | query | Nej |  |
| fromCommentId | string | query | Nej |  |
| viewed | boolean | query | Nej |  |
| type | string | query | Nej |  |

## Svar

Returnerer: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getNotificationCount Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (valgfri)
let urlId = "urlId_example" // String |  (valgfri)
let fromCommentId = "fromCommentId_example" // String |  (valgfri)
let viewed = true // Bool |  (valgfri)
let type = "type_example" // String |  (valgfri)

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

---