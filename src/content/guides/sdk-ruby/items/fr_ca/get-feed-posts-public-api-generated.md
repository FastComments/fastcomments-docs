req
tenantId
afterId

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |
| sso | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeUserInfo | boolean | query | Non |  |

## Réponse

Renvoie: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_feed_posts_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
opts = {
  after_id: 'after_id_example', # Chaîne | 
  limit: 56, # Entier | 
  tags: ['inner_example'], # Tableau<String> | 
  sso: 'sso_example', # Chaîne | 
  is_crawler: true, # Booléen | 
  include_user_info: true # Booléen | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]