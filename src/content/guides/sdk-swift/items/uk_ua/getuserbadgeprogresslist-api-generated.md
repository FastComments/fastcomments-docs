## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadgeProgressList200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressList'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Якщо виникнуть проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необов'язково)
let limit = 987 // Double |  (необов'язково)
let skip = 987 // Double |  (необов'язково)

DefaultAPI.getUserBadgeProgressList(tenantId: tenantId, userId: userId, limit: limit, skip: skip) { (response, error) in
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