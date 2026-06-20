## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | 경로 | 예 |  |
| tenantId | string | 쿼리 | 아니요 |  |

## Response

반환: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateHashTagResponse.php)

## Example

[inline-code-attrs-start title = 'patchHashTag 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현한 클라이언트를 전달하세요.
    // 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // 문자열
$tenant_id = 'tenant_id_example'; // 문자열
$update_hash_tag_body = new \FastComments\Client\Model\UpdateHashTagBody(); // \FastComments\Client\Model\UpdateHashTagBody

try {
    $result = $apiInstance->patchHashTag($tag, $tenant_id, $update_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---