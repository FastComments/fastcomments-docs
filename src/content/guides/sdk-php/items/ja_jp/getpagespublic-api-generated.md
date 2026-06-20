テナントのページを一覧表示します。FChat デスクトップクライアントがルーム一覧を生成するために使用します。
各ページの解決済みカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、リクエストを行ったユーザーのグループアクセスに基づいてフィルタリングされます。

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Optional case-insensitive title prefix filter. |
| sortBy | string | query | No | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |
| hasComments | boolean | query | No | If true, only return pages with at least one comment. |

## レスポンス

返却: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムHTTPクライアントを使用する場合、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | 以前のリクエストから返された `nextCursor` という不透明なページネーションカーソル。`sortBy` と結び付けられます。
$limit = 56; // int | 1..200, default 50
$q = 'q_example'; // string | オプションの大文字小文字を区別しないタイトルの接頭辞フィルタ。
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数の多い順）、または `title`（アルファベット順）。
$has_comments = True; // bool | true の場合、少なくとも1つのコメントがあるページのみ返します。

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]