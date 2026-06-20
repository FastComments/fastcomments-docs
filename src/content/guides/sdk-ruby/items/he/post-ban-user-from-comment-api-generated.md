## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| banEmail | boolean | query | לא |  |
| banEmailDomain | boolean | query | לא |  |
| banIP | boolean | query | לא |  |
| deleteAllUsersComments | boolean | query | לא |  |
| bannedUntil | string | query | לא |  |
| isShadowBan | boolean | query | לא |  |
| updateId | string | query | לא |  |
| banReason | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-post_ban_user_from_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # מחרוזת | 
opts = {
  ban_email: true, # בוליאני | 
  ban_email_domain: true, # בוליאני | 
  ban_ip: true, # בוליאני | 
  delete_all_users_comments: true, # בוליאני | 
  banned_until: 'banned_until_example', # מחרוזת | 
  is_shadow_ban: true, # בוליאני | 
  update_id: 'update_id_example', # מחרוזת | 
  ban_reason: 'ban_reason_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.post_ban_user_from_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]