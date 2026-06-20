---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## 範例

[inline-code-attrs-start title = 'getModerationCommentText 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let commentId = "commentId_example" // 字串 | 
let sso = "sso_example" // 字串 |  (可選)

ModerationAPI.getModerationCommentText(commentId: commentId, sso: sso) { (response, error) in
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