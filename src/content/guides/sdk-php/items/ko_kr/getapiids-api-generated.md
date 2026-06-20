## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filters | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| demo | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentIdsResponse.php)

## 예제

[inline-code-attrs-start title = 'getApiIds 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항이며 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$after_id = 'after_id_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiIds($text_search, $by_ip_from_comment, $filters, $search_filters, $after_id, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiIds: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]