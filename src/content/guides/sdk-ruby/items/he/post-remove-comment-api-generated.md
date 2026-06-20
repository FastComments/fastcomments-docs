## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | נתיב | כן |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/post_remove_comment_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-post_remove_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # מחרוזת | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.post_remove_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_remove_comment: #{e}"
end
[inline-code-end]

---