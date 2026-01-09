## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| page | integer | 쿼리 | 아니오 |  |
| limit | integer | 쿼리 | 아니오 |  |
| skip | integer | 쿼리 | 아니오 |  |
| asTree | boolean | 쿼리 | 아니오 |  |
| skipChildren | integer | 쿼리 | 아니오 |  |
| limitChildren | integer | 쿼리 | 아니오 |  |
| maxTreeDepth | integer | 쿼리 | 아니오 |  |
| urlId | string | 쿼리 | 아니오 |  |
| userId | string | 쿼리 | 아니오 |  |
| anonUserId | string | 쿼리 | 아니오 |  |
| contextUserId | string | 쿼리 | 아니오 |  |
| hashTag | string | 쿼리 | 아니오 |  |
| parentId | string | 쿼리 | 아니오 |  |
| direction | string | 쿼리 | 아니오 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetComments200Response.php)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 56; // int
$limit = 56; // int
$skip = 56; // int
$as_tree = True; // bool
$skip_children = 56; // int
$limit_children = 56; // int
$max_tree_depth = 56; // int
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string
$context_user_id = 'context_user_id_example'; // string
$hash_tag = 'hash_tag_example'; // string
$parent_id = 'parent_id_example'; // string
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections

try {
    $result = $apiInstance->getComments($tenant_id, $page, $limit, $skip, $as_tree, $skip_children, $limit_children, $max_tree_depth, $url_id, $user_id, $anon_user_id, $context_user_id, $hash_tag, $parent_id, $direction);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]