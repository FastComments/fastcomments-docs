запрос
tenantId
afterId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Нет |  |
| limit | integer | query | Нет |  |
| tags | array | query | Нет |  |
| sso | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeUserInfo | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_feed_posts_public200_response.rb)

## Пример

[inline-code-attrs-start title = 'get_feed_posts_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  after_id: 'after_id_example', # String | 
  limit: 56, # Integer | 
  tags: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  is_crawler: true, # Boolean | 
  include_user_info: true # Boolean | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]