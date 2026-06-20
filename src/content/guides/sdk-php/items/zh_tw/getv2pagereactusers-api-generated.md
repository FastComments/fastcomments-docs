## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| id | string | query | 是 |  |

## 回應

回傳: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReactUsersResponse.php)

## 範例

[inline-code-attrs-start title = 'getV2PageReactUsers 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getV2PageReactUsers($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReactUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]