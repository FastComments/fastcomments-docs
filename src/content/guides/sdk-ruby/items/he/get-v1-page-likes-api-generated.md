## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |

## תגובה

מחזיר: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v1_page_likes.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_v1_page_likes'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 

begin
  
  result = api_instance.get_v1_page_likes(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v1_page_likes: #{e}"
end
[inline-code-end]