req
tenantId
afterId

## 參數

| 名稱 | Type | Location | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |
| sso | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeUserInfo | boolean | query | 否 |  |

## 回應

回傳: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## 範例

[inline-code-attrs-start title = 'get_feed_posts_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
opts = {
  after_id: 'after_id_example', # 字串 | 
  limit: 56, # 整數 | 
  tags: ['inner_example'], # 陣列<String> | 
  sso: 'sso_example', # 字串 | 
  is_crawler: true, # 布林值 | 
  include_user_info: true # 布林值 | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]

---