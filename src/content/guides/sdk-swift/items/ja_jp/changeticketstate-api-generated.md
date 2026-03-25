## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeTicketState200Response.swift)

## 例

[inline-code-attrs-start title = 'changeTicketState の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String | 
let id = "id_example" // String | 
let changeTicketStateBody = ChangeTicketStateBody(state: 123) // ChangeTicketStateBody | 

DefaultAPI.changeTicketState(tenantId: tenantId, userId: userId, id: id, changeTicketStateBody: changeTicketStateBody) { (response, error) in
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