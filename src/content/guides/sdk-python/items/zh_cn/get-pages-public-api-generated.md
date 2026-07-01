List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分页游标，作为 `nextCursor` 从之前的请求返回。与相同的 `sortBy` 关联。（可选） |
| limit | integer | query | No | 1..200，默认 50（可选） |
| q | string | query | No | 可选的大小写不敏感的标题前缀过滤器。（可选） |
| sortBy | string | query | No | 排序方式。`updatedAt`（默认，最新的在前），`commentCount`（评论最多的在前），或 `title`（按字母顺序）。（可选） |
| hasComments | boolean | query | No | 若为 true，则仅返回至少有一条评论的页面。（可选） |

## 响应

返回: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 示例

[inline-code-attrs-start title = 'get_pages_public 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 查看 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 不透明的分页游标，作为 `nextCursor` 从之前的请求返回。与相同的 `sortBy` 关联。（可选）
    limit = 56 # int | 1..200，默认 50（可选）
    q = 'q_example' # str | 可选的大小写不敏感的标题前缀过滤器。（可选）
    sort_by = client.PagesSortBy() # PagesSortBy | 排序方式。`updatedAt`（默认，最新的在前），`commentCount`（评论最多的在前），或 `title`（按字母顺序）。（可选）
    has_comments = True # bool | 若为 true，则仅返回至少有一条评论的页面。（可选）

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---