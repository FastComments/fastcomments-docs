## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| editKey | string | query | Nein |  |

## Antwort

Gibt zurück: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'deleteVote Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie sich bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let editKey = "editKey_example" // String |  (optional)

DefaultAPI.deleteVote(tenantId: tenantId, id: id, editKey: editKey) { (response, error) in
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