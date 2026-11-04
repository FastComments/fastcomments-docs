テナントのページ一覧を取得します。FChatデスクトップクライアントがルーム一覧を表示するために使用されます。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSOが必要なページは、リクエストしたユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐付きます。 |
| limit | integer | query | いいえ | 1..200、デフォルトは50 |
| q | string | query | いいえ | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`(デフォルト、新しい順)、`commentCount`(コメント数の多い順)、または `title`(アルファベット順)。 |
| hasComments | boolean | query | いいえ | trueの場合、少なくとも1つのコメントがあるページのみを返します。 |

## レスポンス

戻り値: `GetPublicPagesResponse`

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐付きます。
final limit = 56; // int | 1..200、デフォルトは50
final q = q_example; // String | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。
final sortBy = ; // PagesSortBy | ソート順。`updatedAt`(デフォルト、新しい順)、`commentCount`(コメント数の多い順)、または `title`(アルファベット順)。
final hasComments = true; // bool | trueの場合、少なくとも1つのコメントがあるページのみを返します。

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]