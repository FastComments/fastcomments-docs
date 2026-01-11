## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| postId | string | path | Sì |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/react_feed_post_public200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di react_feed_post_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
post_id = 'post_id_example' # Stringa | 
react_body_params = FastCommentsClient::ReactBodyParams.new # ReactBodyParams | 
opts = {
  is_undo: true, # Booleano | 
  broadcast_id: 'broadcast_id_example', # Stringa | 
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->react_feed_post_public: #{e}"
end
[inline-code-end]