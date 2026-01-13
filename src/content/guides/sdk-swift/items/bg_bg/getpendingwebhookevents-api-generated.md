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

Връща: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са бета. За проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (не е задължително)
let externalId = "externalId_example" // String |  (не е задължително)
let eventType = "eventType_example" // String |  (не е задължително)
let type = "type_example" // String |  (не е задължително)
let domain = "domain_example" // String |  (не е задължително)
let attemptCountGT = 987 // Double |  (не е задължително)
let skip = 987 // Double |  (не е задължително)

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