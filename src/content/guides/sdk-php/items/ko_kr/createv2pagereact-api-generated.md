## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| id | string | query | 예 |  |
| title | string | query | 아니오 |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## 예제

[inline-code-attrs-start title = 'createV2PageReact 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항입니다. `GuzzleHttp\Client`가 기본으로 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열
$id = 'id_example'; // 문자열
$title = 'title_example'; // 문자열

try {
    $result = $apiInstance->createV2PageReact($tenant_id, $url_id, $id, $title);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]