## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'deleteHashTag Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (optional)
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (optional)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
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