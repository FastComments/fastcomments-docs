## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |
| order | string | query | Ne |  |
| after | number | query | Ne |  |
| before | number | query | Ne |  |

## Odziv

Vrne: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Če naletite na težavo, jo prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (neobvezno)
let skip = 987 // Double |  (neobvezno)
let order = SORT_DIR() // SORTDIR |  (neobvezno)
let after = 987 // Double |  (neobvezno)
let before = 987 // Double |  (neobvezno)

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