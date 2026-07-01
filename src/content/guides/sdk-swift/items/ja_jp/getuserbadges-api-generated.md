## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| badgeId | string | query | いいえ |  |
| type | number | query | いいえ |  |
| displayedOnComments | boolean | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## 応答

返却: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserBadges 例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (オプション)
let badgeId = "badgeId_example" // String |  (オプション)
let type = 987 // Double |  (オプション)
let displayedOnComments = true // Bool |  (オプション)
let limit = 987 // Double |  (オプション)
let skip = 987 // Double |  (オプション)

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
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