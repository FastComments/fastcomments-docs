## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| isFlagged | boolean | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-flag_comment_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
comment_id = 'comment_id_example' # מחרוזת | 
is_flagged = true # בוליאני | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->flag_comment_public: #{e}"
end
[inline-code-end]

---