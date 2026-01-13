## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| errorId | string | path | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## 範例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如有需要，取消註解以下以設定 API 金鑰前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$error_id = 'error_id_example'; // string

try {
    $result = $apiInstance->deleteEmailTemplateRenderError($tenant_id, $id, $error_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteEmailTemplateRenderError: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]