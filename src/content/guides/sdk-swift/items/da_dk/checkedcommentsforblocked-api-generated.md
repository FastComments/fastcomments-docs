## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | En kommasepareret liste over kommentar-id'er. |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | En kommasepareret liste over kommentar-id'er.
let sso = "sso_example" // String |  (valgfri)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]