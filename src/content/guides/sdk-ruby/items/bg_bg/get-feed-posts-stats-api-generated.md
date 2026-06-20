## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postIds | array | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/feed_posts_stats_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_feed_posts_stats'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Низ | 
post_ids = ['inner_example'] # Масив<Низ> | 
opts = {
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.get_feed_posts_stats(tenant_id, post_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_stats: #{e}"
end
[inline-code-end]