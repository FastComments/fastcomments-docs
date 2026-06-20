## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад blockFromCommentPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще перебувають у бета-версії. Якщо виникне проблема, будь ласка, повідомте про неї через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let publicBlockFromCommentParams = PublicBlockFromCommentParams(commentIds: ["commentIds_example"]) // PublicBlockFromCommentParams | 
let sso = "sso_example" // String |  (необов'язково)

PublicAPI.blockFromCommentPublic(tenantId: tenantId, commentId: commentId, publicBlockFromCommentParams: publicBlockFromCommentParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]