## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| value | string | 쿼리 | 아니오 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationPageSearchResponse.php)

## 예제

[inline-code-attrs-start title = 'getSearchPages 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->getSearchPages($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]