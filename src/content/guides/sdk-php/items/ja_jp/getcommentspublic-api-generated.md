req
tenantId
urlId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |

## 応答

戻り値: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$options = [
    'page' => 56, // 整数
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // 文字列
    'skip' => 56, // 整数
    'skip_children' => 56, // 整数
    'limit' => 56, // 整数
    'limit_children' => 56, // 整数
    'count_children' => True, // ブール
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // 文字列
    'include_config' => True, // ブール
    'count_all' => True, // ブール
    'includei10n' => True, // ブール
    'locale' => 'locale_example', // 文字列
    'modules' => 'modules_example', // 文字列
    'is_crawler' => True, // ブール
    'include_notification_count' => True, // ブール
    'as_tree' => True, // ブール
    'max_tree_depth' => 56, // 整数
    'use_full_translation_ids' => True, // ブール
    'parent_id' => 'parent_id_example', // 文字列
    'search_text' => 'search_text_example', // 文字列
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // 文字列
    'custom_config_str' => 'custom_config_str_example', // 文字列
    'after_comment_id' => 'after_comment_id_example', // 文字列
    'before_comment_id' => 'before_comment_id_example', // 文字列
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]