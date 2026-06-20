Предишни коментиращи на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след като изчерпате /users/online, за да изобразите секция "Членове".
Курсорно пагиниране по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName} от afterName напред чрез $gt, без разход за $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (изчистен от страна на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за решаване при равенство: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не бъдат изпуснати записи при равни имена. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери с код все още са в бета. За проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор на URL на страницата (изчистен от страна на сървъра).
let afterName = "afterName_example" // String | Курсор: предайте nextAfterName от предишния отговор. (по избор)
let afterUserId = "afterUserId_example" // String | Курсор за решаване при равенство: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не бъдат изпуснати записи при равни имена. (по избор)

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