## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## 예제

[inline-code-attrs-start title = 'getSearchCommentsSummary 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며, 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // 문자열
$filters = 'filters_example'; // 문자열
$search_filters = 'search_filters_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->getSearchCommentsSummary($value, $filters, $search_filters, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]