## 매개변수

| 이름 | Type | Location | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| badgeId | string | query | 아니오 |  |
| type | number | query | 아니오 |  |
| displayedOnComments | boolean | query | 아니오 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadges200Response.php)

## 예제

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 제거하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 문자열
$user_id = 'user_id_example'; // 문자열
$badge_id = 'badge_id_example'; // 문자열
$type = 3.4; // 실수
$displayed_on_comments = True; // 불리언
$limit = 3.4; // 실수
$skip = 3.4; // 실수

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]