## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| state | number | query | Ні |  |
| skip | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще є бета‑версією. У разі будь‑якої проблеми, будь ласка, повідомте за посиланням http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необов’язковий)
let state = 987 // Double |  (необов’язковий)
let skip = 987 // Double |  (необов’язковий)
let limit = 987 // Double |  (необов’язковий)

DefaultAPI.getTickets(tenantId: tenantId, options: DefaultAPI.GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]