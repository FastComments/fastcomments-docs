## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_subscription_a_p_i_response.rb)

## Beispiel

[inline-code-attrs-start title = 'create_subscription Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entfernen Sie das Kommentarzeichen vor der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_api_user_subscription_data = FastCommentsClient::CreateAPIUserSubscriptionData.new({url_id: 'url_id_example'}) # CreateAPIUserSubscriptionData | 

begin
  
  result = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_subscription: #{e}"
end
[inline-code-end]