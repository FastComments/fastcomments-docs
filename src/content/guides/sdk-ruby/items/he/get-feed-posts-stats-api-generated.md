---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| postIds | array | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/feed_posts_stats_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_feed_posts_stats'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
post_ids = ['inner_example'] # מערך<String> | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_feed_posts_stats(tenant_id, post_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_stats: #{e}"
end
[inline-code-end]

---