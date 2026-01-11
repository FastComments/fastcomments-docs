req
tenantId
afterId

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_feed_posts_public200_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_feed_posts_public Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  after_id: 'after_id_example', # String | 
  limit: 56, # Integer | 
  tags: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  is_crawler: true, # Boolean | 
  include_user_info: true # Boolean | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]