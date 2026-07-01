## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| deleteComments | string | query | Nein |  |
| commentDeleteMode | string | query | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'deleteTenantUser Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen bitte melden via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = "deleteComments_example" // String |  (optional)
let commentDeleteMode = "commentDeleteMode_example" // String |  (optional)

DefaultAPI.deleteTenantUser(tenantId: tenantId, id: id, options: DefaultAPI.DeleteTenantUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode)) { (response, error) in
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