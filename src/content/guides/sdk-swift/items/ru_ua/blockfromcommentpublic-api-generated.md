## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Пример

[inline-code-attrs-start title = 'Пример blockFromCommentPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё в бета-версии. При возникновении проблем, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let publicBlockFromCommentParams = PublicBlockFromCommentParams(commentIds: ["commentIds_example"]) // PublicBlockFromCommentParams | 
let sso = "sso_example" // String |  (необязательно)

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

---