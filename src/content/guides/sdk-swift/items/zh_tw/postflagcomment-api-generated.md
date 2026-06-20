## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (可選)

ModerationAPI.postFlagComment(commentId: commentId, sso: sso) { (response, error) in
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