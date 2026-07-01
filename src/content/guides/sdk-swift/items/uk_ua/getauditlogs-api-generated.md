## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |
| order | string | query | Ні |  |
| after | number | query | Ні |  |
| before | number | query | Ні |  |

## Відповідь

Повертає: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getAuditLogs Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. У разі проблеми, будь ласка, повідомте про це за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (optional)
let skip = 987 // Double |  (optional)
let order = SORT_DIR() // SORTDIR |  (optional)
let after = 987 // Double |  (optional)
let before = 987 // Double |  (optional)

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