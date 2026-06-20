## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| locale | string | 查詢 | 否 |  |
| rating | string | 查詢 | 否 |  |
| page | number | 查詢 | 否 |  |

## 回應

回傳: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsTrendingResponse.swift)

## 範例

[inline-code-attrs-start title = 'getGifsTrending 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let locale = "locale_example" // String |  (可選)
let rating = "rating_example" // String |  (可選)
let page = 987 // Double |  (可選)

PublicAPI.getGifsTrending(tenantId: tenantId, locale: locale, rating: rating, page: page) { (response, error) in
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