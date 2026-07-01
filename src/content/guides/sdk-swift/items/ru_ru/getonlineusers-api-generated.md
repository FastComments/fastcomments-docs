Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищенный на сервере). |
| afterName | string | query | No | Курсор: передать nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Разделитель курсора: передать nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы при одинаковых именах записи не пропадали. |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета‑версии. При возникновении проблем, пожалуйста, сообщайте их по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL страницы (очищенный на сервере).
let afterName = "afterName_example" // String | Курсор: передать nextAfterName из предыдущего ответа. (optional)
let afterUserId = "afterUserId_example" // String | Разделитель курсора: передать nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы при одинаковых именах записи не пропадали. (optional)

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