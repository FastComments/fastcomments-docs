## Response

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Example

[inline-code-attrs-start title = 'logoutPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항이며, 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

try {
    $result = $apiInstance->logoutPublic();
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->logoutPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]