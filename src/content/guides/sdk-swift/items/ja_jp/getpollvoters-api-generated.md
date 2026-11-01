Who voted for which option. Anonymous polls never answer, admins-only polls answer only for an admin or moderator.

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## 応答

返却: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## 例

[inline-code-attrs-start title = 'getPollVoters の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
```swift
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (オプション)
let page = 987 // Int |  (オプション)
let search = "search_example" // String |  (オプション)
let sso = "sso_example" // String |  (オプション)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```
[inline-code-end]

---