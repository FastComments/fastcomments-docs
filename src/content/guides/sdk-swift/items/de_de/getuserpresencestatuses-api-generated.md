## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlIdWS | string | query | Ja |  |
| userIds | string | query | Ja |  |

## Antwort

Gibt zurück: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatusesResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getUserPresenceStatuses Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlIdWS = "urlIdWS_example" // String | 
let userIds = "userIds_example" // String | 

PublicAPI.getUserPresenceStatuses(tenantId: tenantId, urlIdWS: urlIdWS, userIds: userIds) { (response, error) in
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