## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| fromName | string | query | Да |  |

## Response

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Example

[inline-code-attrs-start title = 'Пример за send_invite'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на авторизацията
FastCommentsClient.configure do |config|
  # Конфигуриране на авторизацията с помощта на API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Премахнете коментара от следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Низ | 
id = 'id_example' # Низ | 
from_name = 'from_name_example' # Низ | 

begin
  
  result = api_instance.send_invite(tenant_id, id, from_name)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->send_invite: #{e}"
end
[inline-code-end]