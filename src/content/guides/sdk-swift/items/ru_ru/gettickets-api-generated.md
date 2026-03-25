## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| state | number | query | Нет |  |
| skip | number | query | Нет |  |
| limit | number | query | Нет |  |

## Ответ

Возвращает: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Приведенные ниже примеры кода находятся в бета-версии. По любым проблемам сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необязательно)
let state = 987 // Double |  (необязательно)
let skip = 987 // Double |  (необязательно)
let limit = 987 // Double |  (необязательно)

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