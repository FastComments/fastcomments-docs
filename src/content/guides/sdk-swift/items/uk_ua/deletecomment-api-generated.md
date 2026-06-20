---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| contextUserId | string | query | Ні |  |
| isLive | boolean | query | Ні |  |

## Відповідь

Повертає: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще є бета-версією. Якщо виникає проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (необов'язково)
let isLive = true // Bool |  (необов'язково)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, contextUserId: contextUserId, isLive: isLive) { (response, error) in
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