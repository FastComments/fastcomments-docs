## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nee |  |

## Respons

Geeft terug: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_subscription_a_p_i_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'update_subscription Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de volgende regel uit commentaar om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_user_subscription_data = FastCommentsClient::UpdateAPIUserSubscriptionData.new # UpdateAPIUserSubscriptionData | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_subscription: #{e}"
end
[inline-code-end]