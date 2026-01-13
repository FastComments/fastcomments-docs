## Параметры

| Name | Type | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Нет |  |
| externalId | string | query | Нет |  |
| eventType | string | query | Нет |  |
| type | string | query | Нет |  |
| domain | string | query | Нет |  |
| attemptCountGT | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## Пример

[inline-code-attrs-start title = 'getPendingWebhookEvents Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. Для любых проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (необязательно)
let externalId = "externalId_example" // String |  (необязательно)
let eventType = "eventType_example" // String |  (необязательно)
let type = "type_example" // String |  (необязательно)
let domain = "domain_example" // String |  (необязательно)
let attemptCountGT = 987 // Double |  (необязательно)
let skip = 987 // Double |  (необязательно)

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