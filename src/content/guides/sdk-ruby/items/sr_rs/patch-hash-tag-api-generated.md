## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tag | string | path | Да |  |
| tenantId | string | query | Не |  |

## Одговор

Враћа: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_hash_tag200_response.rb)

## Пример

[inline-code-attrs-start title = 'patch_hash_tag Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите овлашћење API кључа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Уклоните коментар са следеће линије да бисте поставили префикс за API кључ, нпр. 'Bearer' (по подразумевању nil)
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