## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-put_close_thread'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
url_id = 'url_id_example' # מחרוזת | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.put_close_thread(url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_close_thread: #{e}"
end
[inline-code-end]

---