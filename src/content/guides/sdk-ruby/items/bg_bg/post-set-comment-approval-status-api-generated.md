## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| approved | boolean | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_comment_approved_response.rb)

## Пример

[inline-code-attrs-start title = 'post_set_comment_approval_status Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Низ |
comment_id = 'comment_id_example' # Низ |
opts = {
  approved: true, # Булев |
  broadcast_id: 'broadcast_id_example', # Низ |
  sso: 'sso_example' # Низ |
}

begin
  
  result = api_instance.post_set_comment_approval_status(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_approval_status: #{e}"
end
[inline-code-end]

---