当前在线的页面查看者：其 websocket 会话当前已订阅该页面的用户。  
返回 anonCount + totalCount（全局房间订阅者，包括我们不枚举的匿名查看者）。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端清理后）。 |
| afterName | string | query | No | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标分割键：传入上一次响应中的 nextAfterUserId。当设置 afterName 时需要此项，以防名称冲突导致条目被丢弃。 |

## Response

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果想使用自定义 HTTP 客户端，传入实现 `GuzzleHttp\ClientInterface` 的客户端实例。
    // 这一步是可选的，默认使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 页面 URL 标识符（服务器端清理后）。
$options = [
    'after_name' => 'after_name_example', // string | 游标：传入上一次响应中的 nextAfterName。
    'after_user_id' => 'after_user_id_example', // string | 游标分割键：传入上一次响应中的 nextAfterUserId。当设置 afterName 时需要此项，以防名称冲突导致条目被丢弃。
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]