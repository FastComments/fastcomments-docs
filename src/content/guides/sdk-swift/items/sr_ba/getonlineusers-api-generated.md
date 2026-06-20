Тренутно-онлајн посматрачи странице: људи чија је websocket сесија претплаћена на страницу у овом тренутку.
Враћа anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор tiebreaker: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како не би дошло до губитка уноса због истих имена. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Пример

[inline-code-attrs-start title = 'getOnlineUsers Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курсор: проследите nextAfterName из претходног одговора. (опционо)
let afterUserId = "afterUserId_example" // String | Курсор tiebreaker: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како не би дошло до губитка уноса због истих имена. (опционо)

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