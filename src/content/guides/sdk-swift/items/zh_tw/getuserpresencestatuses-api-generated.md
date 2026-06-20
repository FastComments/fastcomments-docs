## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlIdWS | string | query | 是 |  |
| userIds | string | query | 是 |  |

## 回應

回傳: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatusesResponse.swift)

## 範例

[inline-code-attrs-start title = 'getUserPresenceStatuses 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為 beta。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // 字串 | 
let urlIdWS = "urlIdWS_example" // 字串 | 
let userIds = "userIds_example" // 字串 | 

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