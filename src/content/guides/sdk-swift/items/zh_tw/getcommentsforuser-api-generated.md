## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| direction | string | query | 否 |  |
| repliesToUserId | string | query | 否 |  |
| page | number | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |

## 回應

回傳：[`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍屬測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let userId = "userId_example" // String |  (可選)
let direction = SortDirections() // SortDirections |  (可選)
let repliesToUserId = "repliesToUserId_example" // String |  (可選)
let page = 987 // Double |  (可選)
let includei10n = true // Bool |  (可選)
let locale = "locale_example" // String |  (可選)
let isCrawler = true // Bool |  (可選)

PublicAPI.getCommentsForUser(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]