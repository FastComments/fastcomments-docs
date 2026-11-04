## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| domain | string | path | Так |  |

## Відповідь

Повертає: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_domain_config_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_domain_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштуйте авторизацію за допомогою API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб задати префікс для API key, наприклад 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain = 'domain_example' # String | 

begin
  
  result = api_instance.delete_domain_config(tenant_id, domain)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_domain_config: #{e}"
end
[inline-code-end]