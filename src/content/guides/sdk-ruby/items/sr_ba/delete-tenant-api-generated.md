## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| sure | string | query | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Пример

[inline-code-attrs-start title = 'delete_tenant Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите ауторизацију помоћу API кључа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откомунтујте следећу линију да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумјевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  sure: 'sure_example' # String | 
}

begin
  
  result = api_instance.delete_tenant(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_tenant: #{e}"
end
[inline-code-end]