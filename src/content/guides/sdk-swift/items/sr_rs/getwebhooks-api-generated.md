Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| event | string | query | Не |  |
| domain | string | query | Не |  |
| source | string | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (опционално)
let domain = "domain_example" // String |  (опционално)
let source = WebhookSource() // WebhookSource |  (опционално)
let skip = 987 // Double |  (опционално)

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