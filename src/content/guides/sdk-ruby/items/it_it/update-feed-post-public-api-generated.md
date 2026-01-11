## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| postId | string | path | Sì |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_feed_post_public200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_feed_post_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
post_id = 'post_id_example' # Stringa | 
update_feed_post_params = FastCommentsClient::UpdateFeedPostParams.new # UpdateFeedPostParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # Stringa | 
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_feed_post_public: #{e}"
end
[inline-code-end]

---