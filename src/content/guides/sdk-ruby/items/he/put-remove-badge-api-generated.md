## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | כן |  |
| userId | string | query | לא |  |
| commentId | string | query | לא |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/remove_user_badge_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה עבור put_remove_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # מחרוזת | 
opts = {
  user_id: 'user_id_example', # מחרוזת | 
  comment_id: 'comment_id_example', # מחרוזת | 
  broadcast_id: 'broadcast_id_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.put_remove_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_remove_badge: #{e}"
end
[inline-code-end]

---