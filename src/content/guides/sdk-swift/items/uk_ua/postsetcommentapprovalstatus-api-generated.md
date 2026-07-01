## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| approved | boolean | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Приклад

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду все ще у бета-версії. У разі будь‑якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let approved = true // Bool | (необов'язково)
let broadcastId = "broadcastId_example" // String | (необов'язково)
let sso = "sso_example" // String | (необов'язково)

ModerationAPI.postSetCommentApprovalStatus(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostSetCommentApprovalStatusOptions(approved: approved, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]