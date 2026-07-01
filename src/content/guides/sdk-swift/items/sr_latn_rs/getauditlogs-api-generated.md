## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | upit | Da |  |
| limit | number | upit | Ne |  |
| skip | number | upit | Ne |  |
| order | string | upit | Ne |  |
| after | number | upit | Ne |  |
| before | number | upit | Ne |  |

## Odgovor

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledići primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (opcionalno)
let skip = 987 // Double |  (opcionalno)
let order = SORT_DIR() // SORTDIR |  (opcionalno)
let after = 987 // Double |  (opcionalno)
let before = 987 // Double |  (opcionalno)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]