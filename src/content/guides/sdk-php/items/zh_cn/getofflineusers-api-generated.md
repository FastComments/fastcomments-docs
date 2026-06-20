在页面上曾发表评论但当前不在线的评论者。按 displayName 排序。
在用尽 /users/online 后使用此项来呈现 "成员" 部分。
基于 commenterName 的游标分页：服务器遍历部分 {tenantId, urlId, commenterName}
index 从 afterName 向前通过 $gt，无 $skip 成本。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识（服务器端清理）。 |
| afterName | string | query | No | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时此项为必需，以防名字相同导致条目丢失。 |

## Response

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果要使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$url_id = 'url_id_example'; // 字符串 | 页面 URL 标识（服务器端清理）。
$after_name = 'after_name_example'; // 字符串 | 游标：传入上一次响应中的 nextAfterName。
$after_user_id = 'after_user_id_example'; // 字符串 | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时此项为必需，以防名字相同导致条目丢失。

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]