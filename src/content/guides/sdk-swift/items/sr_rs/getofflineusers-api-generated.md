Прошли коментатори на страници који ТРЕНУТНО НИСУ на мрежи. Сортирани по displayName.
Користите ово након што исцрпите /users/online за приказ секције „Members".
Пагинација помоћу курсора по commenterName: сервер пролази кроз парцијални {tenantId, urlId, commenterName}
индекс од afterName напред помоћу $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL-а странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Додатни критеријум за курсор: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен, како се уноси са истим именом не би изгубили. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL-а странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курсор: проследите nextAfterName из претходног одговора. (optional)
let afterUserId = "afterUserId_example" // String | Додатни критеријум за курсор: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен, како се уноси са истим именом не би изгубили. (optional)

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