## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nee |  |
| externalId | string | query | Nee |  |
| eventType | string | query | Nee |  |
| type | string | query | Nee |  |
| domain | string | query | Nee |  |
| attemptCountGT | number | query | Nee |  |

## Respons

Geeft terug: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_event_count200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_pending_webhook_event_count Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de volgende regel uit commentaar om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  comment_id: 'comment_id_example', # String | 
  external_id: 'external_id_example', # String | 
  event_type: 'event_type_example', # String | 
  type: 'type_example', # String | 
  domain: 'domain_example', # String | 
  attempt_count_gt: 1.2 # Float | 
}

begin
  
  result = api_instance.get_pending_webhook_event_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_pending_webhook_event_count: #{e}"
end
[inline-code-end]