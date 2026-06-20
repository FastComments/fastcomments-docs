## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_child_comments_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comment_children'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # מחרוזת | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_comment_children(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_comment_children: #{e}"
end
[inline-code-end]

---