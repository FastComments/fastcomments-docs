## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlIdWS | string | query | 是 |  |
| userIds | string | query | 是 |  |

## 回應

回傳: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatuses200Response.php)

## 範例

[inline-code-attrs-start title = 'getUserPresenceStatuses 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$url_id_ws = 'url_id_ws_example'; // 字串
$user_ids = 'user_ids_example'; // 字串

try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]