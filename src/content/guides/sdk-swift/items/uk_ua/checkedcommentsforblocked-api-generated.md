## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentIds | string | query | Так | Список ідентифікаторів коментарів, розділених комами. |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад checkedCommentsForBlocked'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Якщо виникне будь-яка проблема, повідомте про неї через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | Список ідентифікаторів коментарів, розділених комами.
let sso = "sso_example" // String |  (необов'язково)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]