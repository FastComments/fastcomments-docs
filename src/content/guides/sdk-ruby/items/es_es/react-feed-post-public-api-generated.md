## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/react_feed_post_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de react_feed_post_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Cadena | 
post_id = 'post_id_example' # Cadena | 
react_body_params = FastCommentsClient::ReactBodyParams.new # ReactBodyParams | 
opts = {
  is_undo: true, # Booleano | 
  broadcast_id: 'broadcast_id_example', # Cadena | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->react_feed_post_public: #{e}"
end
[inline-code-end]