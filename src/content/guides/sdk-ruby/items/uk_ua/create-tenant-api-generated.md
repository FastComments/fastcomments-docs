## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Повертає: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant200_response.rb)

## Приклад

[inline-code-attrs-start title = 'create_tenant Приклад'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштуйте авторизацію API-ключем: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб задати префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_body = FastCommentsClient::CreateTenantBody.new({name: 'name_example', domain_configuration: [FastCommentsClient::APIDomainConfiguration.new({id: 'id_example', domain: 'domain_example', created_at: Time.now})]}) # CreateTenantBody | 

begin
  
  result = api_instance.create_tenant(tenant_id, create_tenant_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant: #{e}"
end
[inline-code-end]

---