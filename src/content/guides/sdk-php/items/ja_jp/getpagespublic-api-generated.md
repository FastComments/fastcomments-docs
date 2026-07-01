List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.  

テナントのページを一覧取得します。FChat デスクトップクライアントが部屋リストを埋めるために使用します。  
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。  
SSO が必要なページは、リクエストユーザーのグループアクセスに基づいてフィルタリングされます。  

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に関連付けられます。 |
| limit | integer | query | No | 1..200、デフォルトは 50 |
| q | string | query | No | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタ。 |
| sortBy | string | query | No | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | No | true の場合、少なくとも1つのコメントがあるページのみを返します。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に関連付けられます。
    'limit' => 56, // int | 1..200、デフォルトは 50
    'q' => 'q_example', // string | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタ。
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。
    'has_comments' => True, // bool | true の場合、少なくとも1つのコメントがあるページのみを返します。
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]