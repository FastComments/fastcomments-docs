## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Geeft terug: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'update_feed_post Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutel-autorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal het commentaar van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijvoorbeeld 'Bearer' (standaard is nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
feed_post = FastCommentsClient::FeedPost.new({_id: '_id_example', tenant_id: 'tenant_id_example', created_at: Time.now}) # FeedPost | 

begin
  
  result = api_instance.update_feed_post(tenant_id, id, feed_post)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_feed_post: #{e}"
end
[inline-code-end]