## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | path | Sì |  |

## Risposta

Restituisce: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badge_progress_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_badge_progress_by_user_id'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configura l'autorizzazione
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 

begin
  
  result = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badge_progress_by_user_id: #{e}"
end
[inline-code-end]