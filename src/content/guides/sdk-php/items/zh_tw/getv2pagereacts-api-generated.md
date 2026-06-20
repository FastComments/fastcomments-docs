## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |

## 回應

回傳: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReacts.php)

## 範例

[inline-code-attrs-start title = 'getV2PageReacts 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想要使用自訂的 HTTP 用戶端，傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$url_id = 'url_id_example'; // 字串

try {
    $result = $apiInstance->getV2PageReacts($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReacts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]