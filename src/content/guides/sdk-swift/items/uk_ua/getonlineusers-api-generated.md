Наразі онлайн глядачі сторінки: люди, чиї websocket-сесії підписані на сторінку прямо зараз.
Повертає anonCount + totalCount (підписники всієї кімнати, включно з анонімними глядачами, яких ми не перелічуємо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Page URL identifier (очищається на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Додатковий курсор для розв'язання нічийних ситуацій: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи з однаковими іменами не були пропущені. |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Якщо виникнуть проблеми, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Ідентифікатор URL сторінки (очищається на сервері).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
let afterUserId = "afterUserId_example" // String | Додатковий курсор для розв'язання нічийних ситуацій: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи з однаковими іменами не були пропущені. (необов'язково)

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