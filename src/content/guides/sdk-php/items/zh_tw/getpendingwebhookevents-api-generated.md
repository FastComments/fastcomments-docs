## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 否 |  |
| externalId | string | query | 否 |  |
| eventType | string | query | 否 |  |
| type | string | query | 否 |  |
| domain | string | query | 否 |  |
| attemptCountGT | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳： [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEvents200Response.php)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEvents 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如需，取消註解下方以為 API 金鑰設定前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$external_id = 'external_id_example'; // string
$event_type = 'event_type_example'; // string
$type = 'type_example'; // string
$domain = 'domain_example'; // string
$attempt_count_gt = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]