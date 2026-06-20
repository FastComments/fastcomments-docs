---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |
| order | string | query | Не |  |
| after | number | query | Не |  |
| before | number | query | Не |  |

## Отговор

Връща: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета версия. При проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)
let order = SORT_DIR() // SORTDIR |  (по избор)
let after = 987 // Double |  (по избор)
let before = 987 // Double |  (по избор)

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