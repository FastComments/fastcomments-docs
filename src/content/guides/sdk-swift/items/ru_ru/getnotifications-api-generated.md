## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| urlId | string | query | Нет |  |
| fromCommentId | string | query | Нет |  |
| viewed | boolean | query | Нет |  |
| type | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода всё ещё находятся в бета-версии. При любой проблеме, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необязательно)
let urlId = "urlId_example" // String |  (необязательно)
let fromCommentId = "fromCommentId_example" // String |  (необязательно)
let viewed = true // Bool |  (необязательно)
let type = "type_example" // String |  (необязательно)
let skip = 987 // Double |  (необязательно)

DefaultAPI.getNotifications(tenantId: tenantId, options: DefaultAPI.GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip)) { (response, error) in
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