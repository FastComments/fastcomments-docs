## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니요 |  |
| externalId | string | query | 아니요 |  |
| eventType | string | query | 아니요 |  |
| type | string | query | 아니요 |  |
| domain | string | query | 아니요 |  |
| attemptCountGT | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventsResponse.php)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요한 경우 API 키의 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 문자열 (string)
$comment_id = 'comment_id_example'; // 문자열 (string)
$external_id = 'external_id_example'; // 문자열 (string)
$event_type = 'event_type_example'; // 문자열 (string)
$type = 'type_example'; // 문자열 (string)
$domain = 'domain_example'; // 문자열 (string)
$attempt_count_gt = 3.4; // 실수 (float)
$skip = 3.4; // 실수 (float)

try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]