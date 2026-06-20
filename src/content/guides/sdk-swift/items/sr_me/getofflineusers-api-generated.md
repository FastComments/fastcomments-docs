Прошли коментатори на страници који НИСУ тренутно онлајн. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да бисте приказали секцију „Чланови“.
Курсорска пагинација на commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName} од afterName унапред помоћу $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: пошаљите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Ти-брејкер курсора: пошаљите nextAfterUserId из претходног одговора. Потребно када је afterName подешен да се приликом истих имена не би изгубили уноси. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Примјер

[inline-code-attrs-start title = 'getOfflineUsers Примјер'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи узорци кода су још увијек у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Page URL identifier (cleaned server-side).
let afterName = "afterName_example" // String | Cursor: pass nextAfterName from the previous response. (optional)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. (optional)

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