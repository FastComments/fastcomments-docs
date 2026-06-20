## Параметри

| Име | Type | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_feed_post_response.rb)

## Пример

[inline-code-attrs-start title = 'update_feed_post_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Низ | 
post_id = 'post_id_example' # Низ | 
update_feed_post_params = FastCommentsClient::UpdateFeedPostParams.new # UpdateFeedPostParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_feed_post_public: #{e}"
end
[inline-code-end]

---