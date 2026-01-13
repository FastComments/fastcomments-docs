## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Non |  |
| externalId | string | query | Non |  |
| eventType | string | query | Non |  |
| type | string | query | Non |  |
| domain | string | query | Non |  |
| attemptCountGT | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie : [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_events200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pending_webhook_events'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. 'Bearer' (par défaut nil)
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
  attempt_count_gt: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_pending_webhook_events(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_pending_webhook_events: #{e}"
end
[inline-code-end]