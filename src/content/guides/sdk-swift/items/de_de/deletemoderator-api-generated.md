## Parameter

| Name | Type | Location | Required | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| sendEmail | string | query | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'deleteModerator Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie sich bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let sendEmail = "sendEmail_example" // String |  (optional)

DefaultAPI.deleteModerator(tenantId: tenantId, id: id, sendEmail: sendEmail) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]