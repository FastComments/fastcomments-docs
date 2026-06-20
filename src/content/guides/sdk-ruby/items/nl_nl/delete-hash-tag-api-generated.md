## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nee |  |

## Antwoord

Geeft terug: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'delete_hash_tag Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal het commentaar van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  delete_hash_tag_request_body: FastCommentsClient::DeleteHashTagRequestBody.new # DeleteHashTagRequestBody | 
}

begin
  
  result = api_instance.delete_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Fout bij het aanroepen van DefaultApi->delete_hash_tag: #{e}"
end
[inline-code-end]