目前页面上在线的查看者：其 websocket 会话当前订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不逐一列举的匿名观众）。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（在服务器端已清理）。 |
| afterName | string | query | 否 | 游标：从上一个响应中传入 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜项：从上一个响应中传入 nextAfterUserId。当设置 afterName 时需要该值，以免同名导致条目被丢弃。 |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 页面 URL 标识符（在服务器端已清理）。
$after_name = 'after_name_example'; // string | 游标：从上一个响应中传入 nextAfterName。
$after_user_id = 'after_user_id_example'; // string | 游标决胜项：从上一个响应中传入 nextAfterUserId。当设置 afterName 时需要该值，以免同名导致条目被丢弃。

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]