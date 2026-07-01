List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソルです。`sortBy` と同じものに結び付けられます。 |
| limit | integer | query | No | 1..200、デフォルトは 50 |
| q | string | query | No | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタです。 |
| sortBy | string | query | No | ソート順です。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | No | true の場合、少なくとも1つのコメントがあるページのみを返します。 |

## Response

返却: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください。
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソルです。`sortBy` と同じものに結び付けられます。 (optional)
let limit = 987 // Int | 1..200、デフォルトは 50 (optional)
let q = "q_example" // String | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタです。 (optional)
let sortBy = PagesSortBy() // PagesSortBy | ソート順です。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 (optional)
let hasComments = true // Bool | true の場合、少なくとも1つのコメントがあるページのみを返します。 (optional)

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