## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| userId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| repliesToUserId | string | query | 아니오 |  |
| page | number | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$replies_to_user_id = 'replies_to_user_id_example'; // string
$page = 3.4; // float
$includei10n = True; // bool
$locale = 'locale_example'; // string
$is_crawler = True; // bool

try {
    $result = $apiInstance->getCommentsForUser($user_id, $direction, $replies_to_user_id, $page, $includei10n, $locale, $is_crawler);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]