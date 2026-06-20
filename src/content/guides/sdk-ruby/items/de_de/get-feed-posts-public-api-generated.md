req
tenantId
afterId

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Pfad | Ja |  |
| afterId | string | Query | Nein |  |
| limit | integer | Query | Nein |  |
| tags | array | Query | Nein |  |
| sso | string | Query | Nein |  |
| isCrawler | boolean | Query | Nein |  |
| includeUserInfo | boolean | Query | Nein |  |

## Antwort

Gibt zurück: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_feed_posts_public Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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