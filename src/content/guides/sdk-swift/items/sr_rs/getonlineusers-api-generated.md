Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Тренутно онлајн посматрачи странице: људи чија је вебсокет сесија тренутно претплаћена на страницу.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Враћа anonCount + totalCount (претплатнике у соби, укључујући анонимне посматраче које не набрајамо).

## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курзор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курзор решавач везног разлога: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како називне везе не би изгубиле уносе. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курзор: проследите nextAfterName из претходног одговора. (опционо)
let afterUserId = "afterUserId_example" // String | Курзор решавач везног разлога: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како називне везе не би изгубиле уносе. (опционо)

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

---