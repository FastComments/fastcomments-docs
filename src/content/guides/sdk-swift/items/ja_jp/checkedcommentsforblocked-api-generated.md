## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| commentIds | string | クエリ | はい | カンマ区切りのコメントIDの一覧。 |
| sso | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckedCommentsForBlocked200Response.swift)

## 例

[inline-code-attrs-start title = 'checkedCommentsForBlocked の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 次のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | カンマ区切りのコメントIDの一覧。
let sso = "sso_example" // String |  (オプション)

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

---