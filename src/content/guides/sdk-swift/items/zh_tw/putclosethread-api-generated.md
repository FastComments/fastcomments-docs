## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 範例

[inline-code-attrs-start title = 'putCloseThread 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍屬 beta。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (選填)

ModerationAPI.putCloseThread(urlId: urlId, sso: sso) { (response, error) in
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