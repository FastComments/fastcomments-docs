## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 回應

回傳: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetBannedUsersCountResponse.php)

## 範例

[inline-code-attrs-start title = 'getCounts 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是選用的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->getCounts($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCounts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]