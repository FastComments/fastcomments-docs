## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |
| skip | number | query | No |  |

## Response

Returns: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationsResponse.php)

## Example

[inline-code-attrs-start title = 'getNotifications 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configure API key authorization: api_key
// API 키 인증 구성: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 필요에 따라 API 키에 대한 접두사(e.g. Bearer)를 설정하려면 아래 주석을 해제하십시오
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하십시오.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// 문자열
$options = [
    'user_id' => 'user_id_example', // string
    // 문자열
    'url_id' => 'url_id_example', // string
    // 문자열
    'from_comment_id' => 'from_comment_id_example', // string
    // 문자열
    'viewed' => True, // bool
    // 불리언
    'type' => 'type_example', // string
    // 문자열
    'skip' => 3.4, // float
    // 부동소수점
];


try {
    $result = $apiInstance->getNotifications($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]