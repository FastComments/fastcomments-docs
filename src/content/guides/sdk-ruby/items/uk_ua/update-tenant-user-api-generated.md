## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| updateComments | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад update_tenant_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштувати авторизацію API-ключем: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_tenant_user_body = FastCommentsClient::UpdateTenantUserBody.new # UpdateTenantUserBody | 
opts = {
  update_comments: 'update_comments_example' # String | 
}

begin
  
  result = api_instance.update_tenant_user(tenant_id, id, update_tenant_user_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_tenant_user: #{e}"
end
[inline-code-end]