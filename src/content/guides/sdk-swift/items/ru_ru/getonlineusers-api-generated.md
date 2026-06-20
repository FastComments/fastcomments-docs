В настоящий момент онлайн просматривающие страницу: пользователи, чья websocket-сессия в данный момент подписана на страницу.
Возвращает anonCount + totalCount (подписчики комнаты в целом, включая анонимных зрителей, которых мы не перечисляем).

## Parameters

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Разрешитель ничьих для курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда задан afterName, чтобы записи с одинаковыми именами не терялись. |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. В случае проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищается на стороне сервера).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
let afterUserId = "afterUserId_example" // String | Разрешитель ничьих для курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда задан afterName, чтобы записи с одинаковыми именами не терялись. (необязательно)

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