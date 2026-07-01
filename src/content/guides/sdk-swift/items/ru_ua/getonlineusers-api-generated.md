Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищенный на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Cursor tiebreaker: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы при равенстве имён не терялись записи. |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета‑версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищенный на сервере).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы при равенстве имён не терялись записи. (необязательно)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]