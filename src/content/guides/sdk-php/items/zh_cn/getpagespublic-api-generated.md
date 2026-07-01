List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明分页游标，从先前请求的 `nextCursor` 返回。与相同的 `sortBy` 关联。 |
| limit | integer | query | No | 1..200，默认 50 |
| q | string | query | No | 可选的大小写不敏感标题前缀过滤。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（默认，最新的在前），`commentCount`（评论最多的在前），或 `title`（字母顺序）。 |
| hasComments | boolean | query | No | 若为 true，则只返回至少有一条评论的页面。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Optional case-insensitive title prefix filter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
    'has_comments' => True, // bool | If true, only return pages with at least one comment.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]