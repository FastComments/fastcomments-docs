---
Претходни коментатори на страници који нису тренутно онлајн. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да бисте приказали одељак "Чланови".
Курсорска пагинација по commenterName: сервер пролази парцијални индекс {tenantId, urlId, commenterName} од afterName унапред помоћу $gt, без $skip трошка.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор као разрешење: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName подешен тако да се при истим именима записи не би изгубили. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода још увек је у бети. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курсор: проследите nextAfterName из претходног одговора. (опционо)
let afterUserId = "afterUserId_example" // String | Курсор као разрешење: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName подешен тако да се при истим именима записи не би изгубили. (опционо)

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

---