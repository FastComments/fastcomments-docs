テナントのページ一覧を取得します。FChatデスクトップクライアントがルームリストを表示するために使用します。
各ページで、解決されたカスタム設定の `enableFChat` が true である必要があります。
SSOが必要なページは、リクエストしたユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐づきます。 |
| limit | integer | query | いいえ | 1..200、デフォルトは50 |
| q | string | query | いいえ | 任意の、大文字小文字を区別しないタイトルの前方一致フィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`（デフォルト、新しい順）、`commentCount`（コメント数の多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | いいえ | trueの場合、少なくとも1件のコメントがあるページのみ返します。 |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これは任意です。指定しない場合は `GuzzleHttp\Client` がデフォルトで使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | 以前のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に紐づきます。
    'limit' => 56, // int | 1..200、デフォルトは50
    'q' => 'q_example', // string | 任意の、大文字小文字を区別しないタイトルの前方一致フィルター。
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | ソート順。`updatedAt`（デフォルト、新しい順）、`commentCount`（コメント数の多い順）、または `title`（アルファベット順）。
    'has_comments' => True, // bool | trueの場合、少なくとも1件のコメントがあるページのみ返します。
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]