## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| urlId | string | query | Ні |  |
| fromCommentId | string | query | Ні |  |
| viewed | boolean | query | Ні |  |
| type | string | query | Ні |  |

## Відповідь

Повертає: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getNotificationCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необов'язково)
let urlId = "urlId_example" // String |  (необов'язково)
let fromCommentId = "fromCommentId_example" // String |  (необов'язково)
let viewed = true // Bool |  (необов'язково)
let type = "type_example" // String |  (необов'язково)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]