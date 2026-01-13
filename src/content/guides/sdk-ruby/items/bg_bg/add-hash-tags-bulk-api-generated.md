## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Не |  |

## Отговор

Връща: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_hash_tags_bulk200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за add_hash_tags_bulk'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Настройка на удостоверяването
FastCommentsClient.configure do |config|
  # Конфигуриране на удостоверяване с API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
opts = {
  tenant_id: 'tenant_id_example', # String | 
  bulk_create_hash_tags_body: FastCommentsClient::BulkCreateHashTagsBody.new({tags: [FastCommentsClient::BulkCreateHashTagsBodyTagsInner.new({tag: 'tag_example'})]}) # BulkCreateHashTagsBody | 
}

begin
  
  result = api_instance.add_hash_tags_bulk(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tags_bulk: #{e}"
end
[inline-code-end]

---