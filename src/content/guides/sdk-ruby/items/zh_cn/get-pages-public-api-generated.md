列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求每个页面的已解析自定义配置中 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 不透明的分页游标，从先前请求返回为 `nextCursor`。与相同的 `sortBy` 绑定。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的、不区分大小写的标题前缀过滤器。 |
| sortBy | string | query | 否 | 排序顺序。`updatedAt`（默认，最新优先），`commentCount`（评论数最多优先），或 `title`（按字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，则仅返回至少有一条评论的页面。 |

## Response

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## 示例

[inline-code-attrs-start title = 'get_pages_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | 从先前请求返回的模糊分页游标，作为 `nextCursor`。与相同的 `sortBy` 关联。
  limit: 56, # Integer | 1..200，默认 50
  q: 'q_example', # String | 可选的、不区分大小写的标题前缀过滤器。
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | 排序顺序。`updatedAt`（默认，最新优先），`commentCount`（评论数最多优先），或 `title`（按字母顺序）。
  has_comments: true # Boolean | 如果为 true，则仅返回至少有一条评论的页面。
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]