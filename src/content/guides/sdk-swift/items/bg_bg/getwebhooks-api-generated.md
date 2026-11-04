Изброява уебкуцките, конфигурирани за наемателя, както редовете, управлявани от таблото, така и абонаментите чрез API.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| event | string | query | Не |  |
| domain | string | query | Не |  |
| source | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let domain = "domain_example" // String |  (optional)
let source = WebhookSource() // WebhookSource |  (optional)
let skip = 987 // Double |  (optional)

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