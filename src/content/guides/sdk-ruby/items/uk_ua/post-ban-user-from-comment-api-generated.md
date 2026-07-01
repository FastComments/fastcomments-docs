## Параметри

| Назва | Тип | Розташування | Обовʼязковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## Приклад

[inline-code-attrs-start title = 'post_ban_user_from_comment Приклад'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Рядок |
comment_id = 'comment_id_example' # Рядок |
opts = {
  ban_email: true, # Логічний |
  ban_email_domain: true, # Логічний |
  ban_ip: true, # Логічний |
  delete_all_users_comments: true, # Логічний |
  banned_until: 'banned_until_example', # Рядок |
  is_shadow_ban: true, # Логічний |
  update_id: 'update_id_example', # Рядок |
  ban_reason: 'ban_reason_example', # Рядок |
  sso: 'sso_example' # Рядок |
}

begin
  
  result = api_instance.post_ban_user_from_comment(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]