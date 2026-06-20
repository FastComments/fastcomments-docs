## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| repliesToUserId | string | query | いいえ |  |
| page | number | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$replies_to_user_id = 'replies_to_user_id_example'; // string
$page = 3.4; // float
$includei10n = True; // bool
$locale = 'locale_example'; // string
$is_crawler = True; // bool

try {
    $result = $apiInstance->getCommentsForUser($user_id, $direction, $replies_to_user_id, $page, $includei10n, $locale, $is_crawler);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]