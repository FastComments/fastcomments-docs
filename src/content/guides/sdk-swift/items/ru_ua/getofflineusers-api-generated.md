Прошлые комментаторы на странице, которые в данный момент не в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online для отображения раздела "Участники".
Постраничная пагинация (cursor) по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
от afterName вперёд через $gt, без затрат $skip.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Дополнительный ключ для курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName задан, чтобы при совпадении имён записи не терялись. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. Если обнаружите проблему, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищается на стороне сервера).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
let afterUserId = "afterUserId_example" // String | Дополнительный ключ для курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName задан, чтобы при совпадении имён записи не терялись. (необязательно)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]