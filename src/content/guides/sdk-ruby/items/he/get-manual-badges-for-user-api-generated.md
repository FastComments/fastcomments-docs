## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | לא |  |
| commentId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_manual_badges_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_manual_badges_for_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  badges_user_id: 'badges_user_id_example', # מחרוזת | 
  comment_id: 'comment_id_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_manual_badges_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges_for_user: #{e}"
end
[inline-code-end]

---