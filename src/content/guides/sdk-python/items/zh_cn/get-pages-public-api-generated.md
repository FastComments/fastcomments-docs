列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求每个页面的解析自定义配置中的 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 由先前请求返回的 `nextCursor` 不透明分页游标。与相同的 `sortBy` 绑定。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的不区分大小写的标题前缀过滤。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（默认，最新优先）、`commentCount`（评论最多优先）或 `title`（按字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，则只返回至少有一条评论的页面。 |

## 响应

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 示例

[inline-code-attrs-start title = 'get_pages_public 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数的列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端的实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 绑定。(可选)
    limit = 56 # int | 1..200，默认 50 (可选)
    q = 'q_example' # str | 可选的、不区分大小写的标题前缀过滤。 (可选)
    sort_by = client.PagesSortBy() # PagesSortBy | 排序方式。`updatedAt`（默认，最新优先），`commentCount`（评论最多优先），或 `title`（按字母顺序）。(可选)
    has_comments = True # bool | 如果为 true，则只返回至少有一条评论的页面。 (可选)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]