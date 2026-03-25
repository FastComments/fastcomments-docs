## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Одговор

Враћа: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Пример

[inline-code-attrs-start title = 'getTickets Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (опционално)
let state = 987 // Double |  (опционално)
let skip = 987 // Double |  (опционално)
let limit = 987 // Double |  (опционално)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
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