## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| fromCommentId | string | query | 아니오 |  |
| viewed | boolean | query | 아니오 |  |
| type | string | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationsResponse.php)

## 예제

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 지정 http 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$url_id = 'url_id_example'; // string
$from_comment_id = 'from_comment_id_example'; // string
$viewed = True; // bool
$type = 'type_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getNotifications($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]