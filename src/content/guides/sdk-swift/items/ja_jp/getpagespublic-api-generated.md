テナントのページ一覧を取得します。FChatデスクトップクライアントがルーム一覧を表示するために使用します。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSOが必要なページは、リクエストしたユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐づきます。 |
| limit | integer | query | いいえ | 1〜200、デフォルトは50 |
| q | string | query | いいえ | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`(デフォルト、新しい順)、`commentCount`(コメント数の多い順)、または `title`(アルファベット順)。 |
| hasComments | boolean | query | いいえ | trueの場合、少なくとも1つのコメントがあるページのみを返します。 |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐づきます。(optional)
let limit = 987 // Int | 1〜200、デフォルトは50 (optional)
let q = "q_example" // String | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。(optional)
let sortBy = PagesSortBy() // PagesSortBy | ソート順。`updatedAt`(デフォルト、新しい順)、`commentCount`(コメント数の多い順)、または `title`(アルファベット順)。(optional)
let hasComments = true // Bool | trueの場合、少なくとも1つのコメントがあるページのみを返します。(optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]