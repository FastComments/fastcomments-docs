## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
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

## Отговор

Връща: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## Пример

[inline-code-attrs-start title = 'Пример за post_ban_user_from_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Низ | 
opts = {
  ban_email: true, # Булев | 
  ban_email_domain: true, # Булев | 
  ban_ip: true, # Булев | 
  delete_all_users_comments: true, # Булев | 
  banned_until: 'banned_until_example', # Низ | 
  is_shadow_ban: true, # Булев | 
  update_id: 'update_id_example', # Низ | 
  ban_reason: 'ban_reason_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.post_ban_user_from_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]