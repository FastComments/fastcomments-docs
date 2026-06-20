req
tenantId
afterId

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| afterId | string | 查询 | 否 |  |
| limit | integer | 查询 | 否 |  |
| tags | array | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |
| isCrawler | boolean | 查询 | 否 |  |
| includeUserInfo | boolean | 查询 | 否 |  |

## 响应

返回: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## 示例

[inline-code-attrs-start title = 'get_feed_posts_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
opts = {
  after_id: 'after_id_example', # 字符串 | 
  limit: 56, # 整数 | 
  tags: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # 字符串 | 
  is_crawler: true, # 布尔 | 
  include_user_info: true # 布尔 | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]