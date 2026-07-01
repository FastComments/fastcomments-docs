## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|------|------|------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| badgeId | string | query | 아니오 |  |
| type | number | query | 아니오 |  |
| displayedOnComments | boolean | query | 아니오 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## Response

반환: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## Example

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요한 경우 API 키에 대한 접두사 (예: Bearer)를 설정하려면 아래 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 옵션이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 문자열
$options = [
    'user_id' => 'user_id_example', // 문자열
    'badge_id' => 'badge_id_example', // 문자열
    'type' => 3.4, // 부동소수점
    'displayed_on_comments' => True, // 불리언
    'limit' => 3.4, // 부동소수점
    'skip' => 3.4, // 부동소수점
];


try {
    $result = $apiInstance->getUserBadges($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]