## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| approved | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Пример

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Приведённые примеры кода всё ещё в бета-версии. Если возникли проблемы, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let approved = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.postSetCommentApprovalStatus(commentId: commentId, approved: approved, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]