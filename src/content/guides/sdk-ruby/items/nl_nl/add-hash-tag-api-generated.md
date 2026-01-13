## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nee |  |

## Antwoord

Geeft terug: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_hash_tag200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'add_hash_tag Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal het commentaarteken van de volgende regel weg om een prefix voor de API-sleutel in te stellen, bv. 'Bearer' (standaard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
opts = {
  tenant_id: 'tenant_id_example', # String | 
  create_hash_tag_body: FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 
}

begin
  
  result = api_instance.add_hash_tag(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]