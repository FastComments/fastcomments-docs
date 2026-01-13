## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| meta | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenants200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getTenants Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let meta = "meta_example" // String |  (valgfri)
let skip = 987 // Double |  (valgfri)

DefaultAPI.getTenants(tenantId: tenantId, meta: meta, skip: skip) { (response, error) in
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