## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nej |  |
| externalId | string | query | Nej |  |
| eventType | string | query | Nej |  |
| type | string | query | Nej |  |
| domain | string | query | Nej |  |
| attemptCountGT | number | query | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_event_count200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_pending_webhook_event_count Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt godkendelse
FastCommentsClient.configure do |config|
  # Konfigurer API-nøglegodkendelse: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren foran følgende linje for at sætte et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
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