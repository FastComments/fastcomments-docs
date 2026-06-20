## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postId | string | path | 是 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_feed_post_public_response.rb)

## 範例

[inline-code-attrs-start title = 'delete_feed_post_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
post_id = 'post_id_example' # 字串 | 
opts = {
  broadcast_id: 'broadcast_id_example', # 字串 | 
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.delete_feed_post_public(tenant_id, post_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_feed_post_public: #{e}"
end
[inline-code-end]

---