## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| editKey | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## 戻り値

Returns: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIGetCommentTextResponse.swift)

## 例

[inline-code-attrs-start title = 'getCommentText の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (オプション)
let sso = "sso_example" // String |  (オプション)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, editKey: editKey, sso: sso) { (response, error) in
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