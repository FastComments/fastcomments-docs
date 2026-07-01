## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| urlId | string | query | Nej |  |
| fromCommentId | string | query | Nej |  |
| viewed | boolean | query | Nej |  |
| type | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getNotifications Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (valgfri)
let urlId = "urlId_example" // String |  (valgfri)
let fromCommentId = "fromCommentId_example" // String |  (valgfri)
let viewed = true // Bool |  (valgfri)
let type = "type_example" // String |  (valgfri)
let skip = 987 // Double |  (valgfri)

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