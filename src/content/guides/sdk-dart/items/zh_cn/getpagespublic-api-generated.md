# 列出租户的页面  
用于 FChat 桌面客户端填充其房间列表。  
需要在每个页面的解析自定义配置中将 `enableFChat` 设置为 true。  
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的大小写不敏感标题前缀过滤器。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（默认，最新的在前），`commentCount`（评论最多的在前），或 `title`（字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，仅返回至少有一条评论的页面。 |

## 响应

返回: `GetPublicPagesResponse`

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。
final limit = 56; // int | 1..200，默认 50
final q = q_example; // String | 可选的大小写不敏感标题前缀过滤器。
final sortBy = ; // PagesSortBy | 排序方式。`updatedAt`（默认，最新的在前），`commentCount`（评论最多的在前），或 `title`（字母顺序）。
final hasComments = true; // bool | 如果为 true，仅返回至少有一条评论的页面。

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]