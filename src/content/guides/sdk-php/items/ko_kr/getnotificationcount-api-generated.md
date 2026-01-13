## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| fromCommentId | string | query | 아니오 |  |
| viewed | boolean | query | 아니오 |  |
| type | string | query | 아니오 |  |

## 응답

반환: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCount200Response.php)

## 예제

[inline-code-attrs-start title = 'getNotificationCount 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면, `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 문자열
$user_id = 'user_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열
$from_comment_id = 'from_comment_id_example'; // 문자열
$viewed = True; // 부울
$type = 'type_example'; // 문자열

try {
    $result = $apiInstance->getNotificationCount($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]