---
## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationPageSearchResponse.swift)

## 範例

[inline-code-attrs-start title = 'getSearchPages 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍處於測試階段。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let value = "value_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

ModerationAPI.getSearchPages(value: value, sso: sso) { (response, error) in
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