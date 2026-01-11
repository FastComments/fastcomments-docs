## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | En kommasepareret liste over kommentar-id'er. |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckedCommentsForBlocked200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, indberet venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
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