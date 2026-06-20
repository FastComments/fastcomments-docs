## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |
| contextUserId | string | query | 아니오 |  |
| hashTag | string | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| fromDate | integer | query | 아니오 |  |
| toDate | integer | query | 아니오 |  |

## 응답

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 문자열
$page = 56; // 정수
$limit = 56; // 정수
$skip = 56; // 정수
$as_tree = True; // 부울
$skip_children = 56; // 정수
$limit_children = 56; // 정수
$max_tree_depth = 56; // 정수
$url_id = 'url_id_example'; // 문자열
$user_id = 'user_id_example'; // 문자열
$anon_user_id = 'anon_user_id_example'; // 문자열
$context_user_id = 'context_user_id_example'; // 문자열
$hash_tag = 'hash_tag_example'; // 문자열
$parent_id = 'parent_id_example'; // 문자열
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$from_date = 56; // 정수
$to_date = 56; // 정수

try {
    $result = $apiInstance->getComments($tenant_id, $page, $limit, $skip, $as_tree, $skip_children, $limit_children, $max_tree_depth, $url_id, $user_id, $anon_user_id, $context_user_id, $hash_tag, $parent_id, $direction, $from_date, $to_date);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]