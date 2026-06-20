列出租户的页面。FChat 桌面客户端用于填充其房间列表。
要求每个页面的已解析自定义配置中的 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的不区分大小写的标题前缀过滤器。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（默认，按更新时间最新优先），`commentCount`（评论数最多优先），或 `title`（按字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，则仅返回至少有一条评论的页面。 |

## 响应

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。
$limit = 56; // int | 1..200，默认 50
$q = 'q_example'; // string | 可选的不区分大小写的标题前缀过滤器。
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | 排序方式。`updatedAt`（默认，按更新时间最新优先），`commentCount`（评论数最多优先），或 `title`（按字母顺序）。
$has_comments = True; // bool | 如果为 true，则仅返回至少有一条评论的页面。

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]