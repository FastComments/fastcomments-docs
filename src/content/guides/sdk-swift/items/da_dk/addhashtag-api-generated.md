## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTag200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'addHashTag Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, indrapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (valgfri)
let createHashTagBody = CreateHashTagBody(tenantId: "tenantId_example", tag: "tag_example", url: "url_example") // CreateHashTagBody |  (valgfri)

DefaultAPI.addHashTag(tenantId: tenantId, createHashTagBody: createHashTagBody) { (response, error) in
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