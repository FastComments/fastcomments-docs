## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| postIds | array | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_feed_posts_stats200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_feed_posts_stats'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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