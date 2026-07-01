## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## 응답

반환: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCountResponse.php)

## 예시

[inline-code-attrs-start title = 'getNotificationCount 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
// 필요에 따라 API 키에 접두사 (예: Bearer)를 설정하려면 아래 주석을 해제하십시오
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하십시오.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 문자열
$options = [
    'user_id' => 'user_id_example', // 문자열
    'url_id' => 'url_id_example', // 문자열
    'from_comment_id' => 'from_comment_id_example', // 문자열
    'viewed' => True, // 불리언
    'type' => 'type_example', // 문자열
];


try {
    $result = $apiInstance->getNotificationCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]