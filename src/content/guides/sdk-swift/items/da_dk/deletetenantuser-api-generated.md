## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| deleteComments | string | query | Nej |  |
| commentDeleteMode | string | query | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på deleteTenantUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. Ved problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = "deleteComments_example" // String |  (valgfri)
let commentDeleteMode = "commentDeleteMode_example" // String |  (valgfri)

DefaultAPI.deleteTenantUser(tenantId: tenantId, id: id, deleteComments: deleteComments, commentDeleteMode: commentDeleteMode) { (response, error) in
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