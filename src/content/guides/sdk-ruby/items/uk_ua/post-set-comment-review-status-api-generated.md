## Параметри

| Ім'я | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Приклад

[inline-code-attrs-start title = 'post_set_comment_review_status Приклад'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Рядок | 
comment_id = 'comment_id_example' # Рядок | 
opts = {
  reviewed: true, # Булевий | 
  broadcast_id: 'broadcast_id_example', # Рядок | 
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.post_set_comment_review_status(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Помилка під час виклику ModerationApi->post_set_comment_review_status: #{e}"
end
[inline-code-end]