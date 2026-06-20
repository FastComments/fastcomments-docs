## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| skip | integer | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |
| contextUserId | string | query | いいえ |  |
| hashTag | string | query | いいえ |  |
| parentId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| fromDate | integer | query | いいえ |  |
| toDate | integer | query | いいえ |  |

## レスポンス

戻り値: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定します: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには以下のコメントアウトを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string（文字列）
$page = 56; // int（整数）
$limit = 56; // int（整数）
$skip = 56; // int（整数）
$as_tree = True; // bool（真偽値）
$skip_children = 56; // int（整数）
$limit_children = 56; // int（整数）
$max_tree_depth = 56; // int（整数）
$url_id = 'url_id_example'; // string（文字列）
$user_id = 'user_id_example'; // string（文字列）
$anon_user_id = 'anon_user_id_example'; // string（文字列）
$context_user_id = 'context_user_id_example'; // string（文字列）
$hash_tag = 'hash_tag_example'; // string（文字列）
$parent_id = 'parent_id_example'; // string（文字列）
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$from_date = 56; // int（整数）
$to_date = 56; // int（整数）

try {
    $result = $apiInstance->getComments($tenant_id, $page, $limit, $skip, $as_tree, $skip_children, $limit_children, $max_tree_depth, $url_id, $user_id, $anon_user_id, $context_user_id, $hash_tag, $parent_id, $direction, $from_date, $to_date);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]