## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Не |  |

## Одговор

Враћа: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_create_hash_tags_response.rb)

## Пример

[inline-code-attrs-start title = 'add_hash_tags_bulk Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите ауторизацију API кључа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментирајте следећи ред да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумева се nil)
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