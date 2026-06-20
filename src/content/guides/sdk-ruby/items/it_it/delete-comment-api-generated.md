## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Risposta

Restituisce: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_result.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurazione dell'autenticazione
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la linea seguente per impostare un prefisso per la API key, es. 'Bearer' (valore predefinito: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  context_user_id: 'context_user_id_example', # String | 
  is_live: true # Boolean | 
}

begin
  
  result = api_instance.delete_comment(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_comment: #{e}"
end
[inline-code-end]