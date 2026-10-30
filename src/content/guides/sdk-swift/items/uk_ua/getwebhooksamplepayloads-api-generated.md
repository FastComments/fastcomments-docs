Недавні коментарі в точному вигляді, який використовують доставки вебхуків, для створення інтеграцій (наприклад, зразкові дані Zapier). Кожна подія доставляє один і той самий об’єкт коментаря, тому `event` має бути лише дійсним.

## Parameters

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| event | string | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getWebhookSamplePayloads'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще бета. Якщо виникнуть проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let limit = 987 // Double |  (optional)

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
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