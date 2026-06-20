## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|------|------|-------------|
| value | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSiteSearchResponse.php)

## 範例

[inline-code-attrs-start title = 'getSearchSites 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 http 客戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchSites($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSites: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]