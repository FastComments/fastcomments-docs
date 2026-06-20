Колишні коментатори на сторінці, які НЕ перебувають онлайн. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online щоб відобразити секцію "Members".
Курсорна пагінація за commenterName: сервер обходить частковий індекс {tenantId, urlId, commenterName}
Індекс від afterName далі за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищується на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Курсор-тайбрейкер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, якщо встановлено afterName, щоб зв'язки за іменем не призводили до втрати записів. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getOfflineUsers Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. У разі проблеми повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Ідентифікатор URL сторінки (очищується на сервері).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
let afterUserId = "afterUserId_example" // String | Курсор-тайбрейкер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб зв'язки за іменем не призводили до втрати записів. (необов'язково)

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