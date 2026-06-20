## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| direction | string | query | 否 |  |
| repliesToUserId | string | query | 否 |  |
| page | number | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |

## 回應

回傳: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
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