---
В данный момент онлайн-зрители страницы: люди, чья websocket session в настоящее время подписана на страницу.
Возвращает anonCount + totalCount (подписчики на уровне комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы записи с одинаковыми именами не выпали. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся на стадии бета. В случае проблем, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищается на стороне сервера).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
let afterUserId = "afterUserId_example" // String | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы записи с одинаковыми именами не выпали. (необязательно)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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