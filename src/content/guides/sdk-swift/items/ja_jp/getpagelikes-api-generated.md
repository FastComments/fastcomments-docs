ページのいいね数。いいねしたユーザーを含めることもできます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | はい |  |
| includeUserInfo | string | query | いいえ | "public" は非公開プロフィールのユーザーを除外し、"include-private" はそれらを含めます。 |

## レスポンス

戻り値: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## 例

[inline-code-attrs-start title = 'getPageLikes の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new までご報告ください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" は非公開プロフィールのユーザーを除外し、\"include-private\" はそれらを含めます。(optional)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]