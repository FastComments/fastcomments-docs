## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'post_set_comment_spam_status Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Низ | 
comment_id = 'comment_id_example' # Низ | 
opts = {
  spam: true, # Булево | 
  perm_not_spam: true, # Булево | 
  broadcast_id: 'broadcast_id_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.post_set_comment_spam_status(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_spam_status: #{e}"
end
[inline-code-end]