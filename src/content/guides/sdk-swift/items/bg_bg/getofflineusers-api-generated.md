Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Members".
Пагинация с курсор по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почиства се от сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Допълнителен критерий за курса: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропускат записи. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор на URL на страницата (почиства се от сървъра).
let afterName = "afterName_example" // String | Курсор: подайте nextAfterName от предишния отговор. (optional)
let afterUserId = "afterUserId_example" // String | Допълнителен критерий за курса: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропускат записи. (optional)

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