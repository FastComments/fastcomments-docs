## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 示例

[inline-code-attrs-start title = 'replaceTenantPackage 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 配置 API 密钥授权：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消下面注释以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 这是可选的，默认将使用 `GuzzleHttp\Client`
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$id = 'id_example'; // 字符串
$replace_tenant_package_body = new \FastComments\Client\Model\ReplaceTenantPackageBody(); // \FastComments\Client\Model\ReplaceTenantPackageBody

try {
    $result = $apiInstance->replaceTenantPackage($tenant_id, $id, $replace_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->replaceTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]