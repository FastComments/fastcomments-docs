## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| page | number | query | 아니요 |  |
| count | number | query | 아니요 |  |
| text-search | string | query | 아니요 |  |
| byIPFromComment | string | query | 아니요 |  |
| filters | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| sorts | string | query | 아니요 |  |
| demo | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현한 클라이언트를 전달하세요.
    // 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$page = 3.4; // 실수 (float)
$count = 3.4; // 실수 (float)
$text_search = 'text_search_example'; // 문자열 (string)
$by_ip_from_comment = 'by_ip_from_comment_example'; // 문자열 (string)
$filters = 'filters_example'; // 문자열 (string)
$search_filters = 'search_filters_example'; // 문자열 (string)
$sorts = 'sorts_example'; // 문자열 (string)
$demo = True; // 불리언 (bool)
$sso = 'sso_example'; // 문자열 (string)

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]