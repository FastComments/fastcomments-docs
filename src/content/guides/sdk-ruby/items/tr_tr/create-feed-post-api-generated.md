## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| broadcastId | string | query | Hayır |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| skipDupCheck | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_feed_posts_response.rb)

## Örnek

[inline-code-attrs-start title = 'create_feed_post Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi yapılandır
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Aşağıdaki satırın yorumunu kaldırarak API anahtarı için bir önek ayarlayın, örn. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_feed_post_params = FastCommentsClient::CreateFeedPostParams.new # CreateFeedPostParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # String | 
  is_live: true, # Boolean | 
  do_spam_check: true, # Boolean | 
  skip_dup_check: true # Boolean | 
}

begin
  
  result = api_instance.create_feed_post(tenant_id, create_feed_post_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_feed_post: #{e}"
end
[inline-code-end]