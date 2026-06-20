## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| errorId | string | path | Ja |  |

## Response

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'delete_email_template_render_error Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API key-autorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal het commentaar van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard is nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
error_id = 'error_id_example' # String | 

begin
  
  result = api_instance.delete_email_template_render_error(tenant_id, id, error_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_email_template_render_error: #{e}"
end
[inline-code-end]