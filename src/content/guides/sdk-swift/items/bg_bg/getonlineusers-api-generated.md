Понастоящем онлайн зрители на страница: хора, чиито websocket сесии са абонирани за страницата в момента.
Връща anonCount + totalCount (абонати в цялата стая, включително анонимни зрители, които не изброяваме).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почистен от страна на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за разрешаване на равни стойности: предайте nextAfterUserId от предишния отговор. Изисква се когато afterName е зададено, за да не се изпускат записи при еднакви имена. |

## Response

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор на URL на страницата (почистен от страна на сървъра).
let afterName = "afterName_example" // String | Курсор: предайте nextAfterName от предишния отговор. (по избор)
let afterUserId = "afterUserId_example" // String | Курсор за разрешаване на равни стойности: предайте nextAfterUserId от предишния отговор. Изисква се когато afterName е зададено, за да не се изпускат записи при еднакви имена. (по избор)

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