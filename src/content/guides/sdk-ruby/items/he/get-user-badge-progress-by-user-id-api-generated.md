## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | path | כן |  |

## תגובה

מחזיר: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badge_progress_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_badge_progress_by_user_id'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # הגדר אישור מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # בטל את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל 'Bearer' (ברירת מחדל: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 

begin
  
  result = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badge_progress_by_user_id: #{e}"
end
[inline-code-end]