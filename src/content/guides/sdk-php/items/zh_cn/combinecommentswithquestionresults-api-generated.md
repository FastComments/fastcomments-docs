---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |
| minValue | number | query | 否 |  |
| maxValue | number | query | 否 |  |
| limit | number | query | 否 |  |

## 响应

返回: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineQuestionResultsWithCommentsResponse.php)

## 示例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消下面注释以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$question_id = 'question_id_example'; // 字符串
$question_ids = array('question_ids_example'); // 字符串[]
$url_id = 'url_id_example'; // 字符串
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // 布尔
$min_value = 3.4; // 浮点数
$max_value = 3.4; // 浮点数
$limit = 3.4; // 浮点数

try {
    $result = $apiInstance->combineCommentsWithQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $start_date, $force_recalculate, $min_value, $max_value, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->combineCommentsWithQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---