## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| sendEmail | string | query | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## 例

[inline-code-attrs-start title = 'deleteModerator の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let sendEmail = "sendEmail_example" // String |  (省略可)

DefaultAPI.deleteModerator(tenantId: tenantId, id: id, sendEmail: sendEmail) { (response, error) in
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