## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回：[`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 配置 API 密钥授权: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 取消注释以下内容以设置 API 密钥的前缀（例如 Bearer），如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 字符串
$options = [
    'url_id' => 'url_id_example', // 字符串
    'user_id' => 'user_id_example', // 字符串
    'start_date' => 'start_date_example', // 字符串
    'question_id' => 'question_id_example', // 字符串
    'question_ids' => 'question_ids_example', // 字符串
    'skip' => 3.4, // 浮点数
];


try {
    $result = $apiInstance->getQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]