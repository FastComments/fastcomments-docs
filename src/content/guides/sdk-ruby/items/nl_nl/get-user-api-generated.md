## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwoord

Geeft terug: [`GetUser200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_user Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaar van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_user(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user: #{e}"
end
[inline-code-end]