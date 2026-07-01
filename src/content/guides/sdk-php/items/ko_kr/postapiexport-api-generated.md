## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## 예시

[inline-code-attrs-start title = 'postApiExport 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 문자열
$options = [
    'text_search' => 'text_search_example', // 문자열
    'by_ip_from_comment' => 'by_ip_from_comment_example', // 문자열
    'filters' => 'filters_example', // 문자열
    'search_filters' => 'search_filters_example', // 문자열
    'sorts' => 'sorts_example', // 문자열
    'sso' => 'sso_example', // 문자열
];


try {
    $result = $apiInstance->postApiExport($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'ModerationApi->postApiExport 호출 중 예외 발생: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]