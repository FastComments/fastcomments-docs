---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери на код все още са в бета. При проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (по избор)
let externalId = "externalId_example" // String |  (по избор)
let eventType = "eventType_example" // String |  (по избор)
let type = "type_example" // String |  (по избор)
let domain = "domain_example" // String |  (по избор)
let attemptCountGT = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
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