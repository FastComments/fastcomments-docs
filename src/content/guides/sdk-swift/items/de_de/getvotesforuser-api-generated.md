## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|-------------|--------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotesForUser200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'getVotesForUser Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userId = "userId_example" // String |  (optional)
let anonUserId = "anonUserId_example" // String |  (optional)

DefaultAPI.getVotesForUser(tenantId: tenantId, urlId: urlId, userId: userId, anonUserId: anonUserId) { (response, error) in
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