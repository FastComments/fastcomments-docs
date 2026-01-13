## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_configs200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_question_configs Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaartekens van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_configs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_configs: #{e}"
end
[inline-code-end]