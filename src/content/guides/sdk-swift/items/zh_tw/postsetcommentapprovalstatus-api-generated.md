## 參數

| 名稱 | 類型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| sso | string | query | No |  |

## 回應

傳回： [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## 範例

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍處於測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let commentId = "commentId_example" // 字串 | 
let approved = true // 布林 |  (選用)
let sso = "sso_example" // 字串 |  (選用)

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

---