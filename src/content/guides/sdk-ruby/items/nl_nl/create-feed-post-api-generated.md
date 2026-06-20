## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nee |  |
| isLive | boolean | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| skipDupCheck | boolean | query | Nee |  |

## Respons

Retourneert: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_feed_posts_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'create_feed_post Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaartekens weg van de volgende regel om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard is nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_feed_post_params = FastCommentsClient::CreateFeedPostParams.new # CreateFeedPostParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # String | 
  is_live: true, # Boolean | 
  do_spam_check: true, # Boolean | 
  skip_dup_check: true # Boolean | 
}

begin
  
  result = api_instance.create_feed_post(tenant_id, create_feed_post_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_feed_post: #{e}"
end
[inline-code-end]