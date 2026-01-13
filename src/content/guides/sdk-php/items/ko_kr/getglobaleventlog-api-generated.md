req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| userIdWS | string | query | 예 |  |
| startTime | integer | query | 예 |  |
| endTime | integer | query | 예 |  |

## 응답

반환: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLog200Response.php)

## 예제

[inline-code-attrs-start title = 'getGlobalEventLog 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이 옵션은 선택 사항이며 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열
$user_id_ws = 'user_id_ws_example'; // 문자열
$start_time = 56; // 정수
$end_time = 56; // 정수

try {
    $result = $apiInstance->getGlobalEventLog($tenant_id, $url_id, $user_id_ws, $start_time, $end_time);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGlobalEventLog: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]