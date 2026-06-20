## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nej |  |
| isLive | boolean | query | Nej |  |

## Svar

Returnerer: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Eksempel

[inline-code-attrs-start title = 'deleteComment Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (valgfri)
let isLive = true // Bool |  (valgfri)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, contextUserId: contextUserId, isLive: isLive) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]