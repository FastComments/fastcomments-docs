## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |

## Antwoord

Retourneert: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_votes200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_votes Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaartekens weg van de volgende regel om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard is nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 

begin
  
  result = api_instance.get_votes(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_votes: #{e}"
end
[inline-code-end]

---