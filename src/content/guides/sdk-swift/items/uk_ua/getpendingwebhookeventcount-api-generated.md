## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEventCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета-версії. У разі проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (необов'язково)
let externalId = "externalId_example" // String |  (необов'язково)
let eventType = "eventType_example" // String |  (необов'язково)
let type = "type_example" // String |  (необов'язково)
let domain = "domain_example" // String |  (необов'язково)
let attemptCountGT = 987 // Double |  (необов'язково)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]