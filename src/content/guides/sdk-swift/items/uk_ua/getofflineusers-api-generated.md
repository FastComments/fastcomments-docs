---
Колишні коментатори на сторінці, які НЕ перебувають онлайн. Впорядковано за displayName.  
Використовуйте це після вичерпання /users/online, щоб відобразити розділ “Members”.  
Пагінація курсором за commenterName: сервер проходить частковий {tenantId, urlId, commenterName} індекс від afterName вперед за допомогою $gt, без витрат $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищений на боці сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Тай-брейкер курсору: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб уникнути пропуску записів через однакові імена. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще в beta-версії. У разі проблем, будь ласка, повідомляйте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Ідентифікатор URL сторінки (очищений на боці сервера).
let afterName = "afterName_example" // String | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
let afterUserId = "afterUserId_example" // String | Тай-брейкер курсору: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб уникнути пропуску записів через однакові імена. (необов'язково)

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

---