## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tag | string | path | Так |  |
| tenantId | string | query | Ні |  |

## Відповідь

Повертає: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_hash_tag_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад patch_hash_tag'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштувати авторизацію за API-ключем: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  update_hash_tag_body: FastCommentsClient::UpdateHashTagBody.new # UpdateHashTagBody | 
}

begin
  
  result = api_instance.patch_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_hash_tag: #{e}"
end
[inline-code-end]