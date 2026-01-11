## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nej |  |
| skip | number | query | Nej |  |
| order | string | query | Nej |  |
| after | number | query | Nej |  |
| before | number | query | Nej |  |

## Svar

Returnerer: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getAuditLogs Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Ved problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (valgfri)
let skip = 987 // Double |  (valgfri)
let order = SORT_DIR() // SORTDIR |  (valgfri)
let after = 987 // Double |  (valgfri)
let before = 987 // Double |  (valgfri)

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
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