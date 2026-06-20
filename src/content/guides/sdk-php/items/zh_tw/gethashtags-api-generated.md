## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| page | number | query | 否 |  |

## 回應

回傳: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTagsResponse.php)

## 範例

[inline-code-attrs-start title = 'getHashTags 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如有需要，取消註解下面以設定前綴（例如 Bearer）給 API 金鑰
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，`GuzzleHttp\Client` 會被用作預設。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字串
$page = 3.4; // 浮點數

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]