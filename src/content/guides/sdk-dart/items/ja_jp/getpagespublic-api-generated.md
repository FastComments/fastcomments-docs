List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐付く。 |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタ。 |
| sortBy | string | query | No | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | No | true の場合、少なくとも1つのコメントがあるページのみを返す。 |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐付く。
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタ。
final sortBy = ; // PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。
final hasComments = true; // bool | true の場合、少なくとも1つのコメントがあるページのみを返す。

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]