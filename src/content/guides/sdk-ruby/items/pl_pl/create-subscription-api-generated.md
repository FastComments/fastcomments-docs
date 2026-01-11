## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Response

Zwraca: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_subscription_a_p_i_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład create_subscription'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Skonfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
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

---