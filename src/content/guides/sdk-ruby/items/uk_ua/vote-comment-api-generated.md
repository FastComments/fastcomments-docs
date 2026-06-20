## Параметри

| Назва | Тип | Location | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| sessionId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад vote_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Рядок | 
comment_id = 'comment_id_example' # Рядок | 
url_id = 'url_id_example' # Рядок | 
broadcast_id = 'broadcast_id_example' # Рядок | 
vote_body_params = FastCommentsClient::VoteBodyParams.new({commenter_email: 'commenter_email_example', commenter_name: 'commenter_name_example', vote_dir: 'up', url: 'url_example'}) # VoteBodyParams | 
opts = {
  session_id: 'session_id_example', # Рядок | 
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->vote_comment: #{e}"
end
[inline-code-end]