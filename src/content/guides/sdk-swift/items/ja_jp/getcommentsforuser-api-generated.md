---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| repliesToUserId | string | query | いいえ |  |
| page | number | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let userId = "userId_example" // String |  (任意)
let direction = SortDirections() // SortDirections |  (任意)
let repliesToUserId = "repliesToUserId_example" // String |  (任意)
let page = 987 // Double |  (任意)
let includei10n = true // Bool |  (任意)
let locale = "locale_example" // String |  (任意)
let isCrawler = true // Bool |  (任意)

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

---