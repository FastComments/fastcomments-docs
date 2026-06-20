テナントのページ一覧を取得します。FChat デスクトップクライアントがルーム一覧を表示するために使用します。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、リクエストユーザーのグループアクセスに基づいてフィルタされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 前のリクエストの `nextCursor` として返される不透明なページネーションカーソルです。同じ `sortBy` に紐づきます。 |
| limit | integer | query | いいえ | 1..200、デフォルトは50 |
| q | string | query | いいえ | 省略可能な大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多いものが先）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | いいえ | true の場合、コメントが少なくとも1件あるページのみを返します。 |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 前のリクエストの `nextCursor` として返される不透明なページネーションカーソル。 同じ `sortBy` に紐づきます。 (省略可)
let limit = 987 // Int | 1..200、デフォルト 50 (省略可)
let q = "q_example" // String | 省略可能な大文字小文字を区別しないタイトルのプレフィックスフィルター。 (省略可)
let sortBy = PagesSortBy() // PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多いものが先）、または `title`（アルファベット順）。 (省略可)
let hasComments = true // Bool | true の場合、コメントが少なくとも1件あるページのみを返します。 (省略可)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]