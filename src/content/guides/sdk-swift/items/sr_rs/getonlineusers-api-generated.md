Тренутно онлајн гледаоци странице: људи чија websocket сесија је тренутно претплаћена на страницу.
Враћа anonCount + totalCount (претплатници у целој соби, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор за разликовање везаних случајева: проследите nextAfterUserId из претходног одговора. Потребно када је afterName подешен да би се спречило изостављање записа при истим именима. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Пример

[inline-code-attrs-start title = 'getOnlineUsers Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још увек у бета фази. За било који проблем, пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курсор: проследите nextAfterName из претходног одговора. (опционо)
let afterUserId = "afterUserId_example" // String | Курсор за разликовање везаних случајева: проследите nextAfterUserId из претходног одговора. Потребно када је afterName подешен да би се спречило изостављање записа при истим именима. (опционо)

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