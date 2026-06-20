## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| contextUserId | string | query | 아니요 |  |
| isLive | boolean | query | 아니요 |  |

## Response

반환: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentResult.php)

## 예제

[inline-code-attrs-start title = 'deleteComment 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$context_user_id = 'context_user_id_example'; // string
$is_live = True; // bool

try {
    $result = $apiInstance->deleteComment($tenant_id, $id, $context_user_id, $is_live);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]