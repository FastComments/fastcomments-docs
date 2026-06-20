## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationPageSearchResponse.php)

## 範例

[inline-code-attrs-start title = 'getSearchPages 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$value = 'value_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->getSearchPages($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]