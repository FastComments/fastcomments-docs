## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| banEmail | boolean | query | नहीं |  |
| banEmailDomain | boolean | query | नहीं |  |
| banIP | boolean | query | नहीं |  |
| deleteAllUsersComments | boolean | query | नहीं |  |
| bannedUntil | string | query | नहीं |  |
| isShadowBan | boolean | query | नहीं |  |
| updateId | string | query | नहीं |  |
| banReason | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## उदाहरण

[inline-code-attrs-start title = 'post_ban_user_from_comment उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
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
  
  result = api_instance.post_ban_user_from_comment(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]