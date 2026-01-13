## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Нет |  |

## Ответ

Возвращает: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_hash_tag200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример add_hash_tag'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
opts = {
  tenant_id: 'tenant_id_example', # String | 
  create_hash_tag_body: FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 
}

begin
  
  result = api_instance.add_hash_tag(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]

---