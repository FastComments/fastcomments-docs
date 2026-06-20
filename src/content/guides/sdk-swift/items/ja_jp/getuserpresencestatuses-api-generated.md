---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| urlIdWS | string | クエリ | はい |  |
| userIds | string | クエリ | はい |  |

## レスポンス

返却: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatusesResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserPresenceStatuses の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlIdWS = "urlIdWS_example" // String | 
let userIds = "userIds_example" // String | 

PublicAPI.getUserPresenceStatuses(tenantId: tenantId, urlIdWS: urlIdWS, userIds: userIds) { (response, error) in
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