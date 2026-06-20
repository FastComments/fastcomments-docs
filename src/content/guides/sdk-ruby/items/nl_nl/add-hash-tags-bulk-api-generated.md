## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nee |  |

## Respons

Geeft terug: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_create_hash_tags_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'add_hash_tags_bulk Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # API-sleutelautorisatie configureren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaarstreep van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
opts = {
  tenant_id: 'tenant_id_example', # String | 
  bulk_create_hash_tags_body: FastCommentsClient::BulkCreateHashTagsBody.new({tags: [FastCommentsClient::BulkCreateHashTagsBodyTagsInner.new({tag: 'tag_example'})]}) # BulkCreateHashTagsBody | 
}

begin
  
  result = api_instance.add_hash_tags_bulk(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tags_bulk: #{e}"
end
[inline-code-end]