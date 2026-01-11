## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| voteId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| editKey | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_comment_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Рядок | 
comment_id = 'comment_id_example' # Рядок | 
vote_id = 'vote_id_example' # Рядок | 
url_id = 'url_id_example' # Рядок | 
broadcast_id = 'broadcast_id_example' # Рядок | 
opts = {
  edit_key: 'edit_key_example', # Рядок | 
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]