## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_subscription_a_p_i_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример update_subscription'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка авторизации
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Раскомментируйте следующую строку, чтобы задать префикс для API-ключа, например 'Bearer' (по умолчанию nil)
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