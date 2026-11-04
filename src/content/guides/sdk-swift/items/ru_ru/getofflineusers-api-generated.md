Прошлые комментаторы страницы, которые НЕ в сети в данный момент. Отсортированы по displayName.
Используйте этот метод после исчерпания /users/online для отображения раздела «Участники».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName} от afterName вперёд через $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы совпадения имён не приводили к потере записей. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищается на сервере).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName из предыдущего ответа. (optional)
let afterUserId = "afterUserId_example" // String | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы совпадения имён не приводили к потере записей. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]