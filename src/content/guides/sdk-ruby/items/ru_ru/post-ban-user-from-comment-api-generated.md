## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Нет |  |
| banEmailDomain | boolean | query | Нет |  |
| banIP | boolean | query | Нет |  |
| deleteAllUsersComments | boolean | query | Нет |  |
| bannedUntil | string | query | Нет |  |
| isShadowBan | boolean | query | Нет |  |
| updateId | string | query | Нет |  |
| banReason | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## Пример

[inline-code-attrs-start title = 'Пример post_ban_user_from_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  ban_email: true, # Boolean | 
  ban_email_domain: true, # Boolean | 
  ban_ip: true, # Boolean | 
  delete_all_users_comments: true, # Boolean | 
  banned_until: 'banned_until_example', # String | 
  is_shadow_ban: true, # Boolean | 
  update_id: 'update_id_example', # String | 
  ban_reason: 'ban_reason_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_ban_user_from_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]