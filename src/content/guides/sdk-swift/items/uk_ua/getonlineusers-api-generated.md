Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
**Поточні онлайн‑переглядачі сторінки: люди, чия WebSocket‑сесія підписана на сторінку саме зараз.**

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
**Повертає anonCount + totalCount (підписники по всій кімнаті, включаючи анонімних переглядачів, яких ми не перераховуємо).**

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | **Ідентифікатор URL сторінки (очищений на сервері).** |
| afterName | string | query | No | **Курсор: передайте nextAfterName з попередньої відповіді.** |
| afterUserId | string | query | No | **Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб уникнути пропуску записів через однакові імена.** |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще перебувають у бета‑версії. У разі проблеми, будь ласка, повідомте про це за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | **Ідентифікатор URL сторінки (очищений на сервері).**
let afterName = "afterName_example" // String | **Курсор: передайте nextAfterName з попередньої відповіді.** (optional)
let afterUserId = "afterUserId_example" // String | **Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб уникнути пропуску записів через однакові імена.** (optional)

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