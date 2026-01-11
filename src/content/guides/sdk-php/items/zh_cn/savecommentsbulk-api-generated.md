## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| isLive | boolean | 查询 | 否 |  |
| doSpamCheck | boolean | 查询 | 否 |  |
| sendEmails | boolean | 查询 | 否 |  |
| populateNotifications | boolean | 查询 | 否 |  |

## 响应

返回: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveComment200Response.php)

## 示例

[inline-code-attrs-start title = 'saveCommentsBulk 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果你想使用自定义 http 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_comment_params = array(new \FastComments\Client\Model\CreateCommentParams()); // \FastComments\Client\Model\CreateCommentParams[]
$is_live = True; // bool
$do_spam_check = True; // bool
$send_emails = True; // bool
$populate_notifications = True; // bool

try {
    $result = $apiInstance->saveCommentsBulk($tenant_id, $create_comment_params, $is_live, $do_spam_check, $send_emails, $populate_notifications);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveCommentsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]