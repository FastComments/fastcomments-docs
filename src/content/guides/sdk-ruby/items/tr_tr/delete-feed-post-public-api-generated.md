## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| postId | string | path | Evet |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_feed_post_public_response.rb)

## Örnek

[inline-code-attrs-start title = 'delete_feed_post_public Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Dize | 
post_id = 'post_id_example' # Dize | 
opts = {
  broadcast_id: 'broadcast_id_example', # Dize | 
  sso: 'sso_example' # Dize | 
}

begin
  
  result = api_instance.delete_feed_post_public(tenant_id, post_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_feed_post_public: #{e}"
end
[inline-code-end]