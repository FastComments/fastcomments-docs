---
页面上过去的评论者（当前不在线）。按 displayName 排序。  
在耗尽 /users/online 之后使用，以渲染“成员”部分。  
对 commenterName 进行游标分页：服务器从部分 `{tenantId, urlId, commenterName}` 索引在 `afterName` 之后通过 `$gt` 前进，无 `$skip` 开销。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|------|------|------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（服务器端清理后）。 |
| afterName | string | query | 否 | 游标：传入前一次响应中的 `nextAfterName`。 |
| afterUserId | string | query | 否 | 游标分割键：传入前一次响应中的 `nextAfterUserId`。当 `afterName` 已设置时必须提供，以防名称相同导致条目被丢弃。 |

## 响应

返回: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 示例

[inline-code-attrs-start title = '获取离线用户示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 页面 URL 标识符（服务器端清理后）。
$options = [
    'after_name' => 'after_name_example', // string | 游标：传入前一次响应中的 nextAfterName。
    'after_user_id' => 'after_user_id_example', // string | 游标分割键：传入前一次响应中的 nextAfterUserId。当 afterName 已设置时必须提供，以防名称相同导致条目被丢弃。
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]