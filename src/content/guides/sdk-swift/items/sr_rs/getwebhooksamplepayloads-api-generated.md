---
Скори коментари у тачно истом облику који користе испоруке вебхука, за изградњу интеграција (на пример Zapier пример података). Сваки догађај испоручује исти објекат коментара, тако да `event` мора бити валидан.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Одговор

Враћа: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer getWebhookSamplePayloads'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (опционално)
let limit = 987 // Double |  (опционално)

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