## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| postId | string | path | Tak |  |
| isUndo | boolean | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/react_feed_post_public200_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład react_feed_post_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
post_id = 'post_id_example' # String | 
react_body_params = FastCommentsClient::ReactBodyParams.new # ReactBodyParams | 
opts = {
  is_undo: true, # Boolean | 
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->react_feed_post_public: #{e}"
end
[inline-code-end]