## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postIds | array | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UserReactsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getUserReactsPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為 beta 版本。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] |  (選填)
let sso = "sso_example" // String |  (選填)

PublicAPI.getUserReactsPublic(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
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