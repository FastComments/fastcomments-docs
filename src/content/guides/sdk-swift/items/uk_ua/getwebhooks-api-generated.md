Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| event | string | query | Ні |  |
| domain | string | query | Ні |  |
| source | string | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getWebhooks Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду все ще у бета-версії. У разі будь-якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (необов'язково)
let domain = "domain_example" // String |  (необов'язково)
let source = WebhookSource() // WebhookSource |  (необов'язково)
let skip = 987 // Double |  (необов'язково)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]