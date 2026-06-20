## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример update_feed_post'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите API кључ за ауторизацију: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментујте следећи ред да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
feed_post = FastCommentsClient::FeedPost.new({_id: '_id_example', tenant_id: 'tenant_id_example', created_at: Time.now}) # FeedPost | 

begin
  
  result = api_instance.update_feed_post(tenant_id, id, feed_post)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_feed_post: #{e}"
end
[inline-code-end]