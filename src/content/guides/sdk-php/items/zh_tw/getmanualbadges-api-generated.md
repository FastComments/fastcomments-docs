## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantManualBadgesResponse.php)

## 範例

[inline-code-attrs-start title = 'getManualBadges 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadges($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---