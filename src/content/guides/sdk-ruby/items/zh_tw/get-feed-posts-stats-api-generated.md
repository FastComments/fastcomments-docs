## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postIds | array | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/feed_posts_stats_response.rb)

## 範例

[inline-code-attrs-start title = 'get_feed_posts_stats 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
post_ids = ['inner_example'] # Array<String> | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_feed_posts_stats(tenant_id, post_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_stats: #{e}"
end
[inline-code-end]

---