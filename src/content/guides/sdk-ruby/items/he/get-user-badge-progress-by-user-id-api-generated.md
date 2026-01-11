## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | path | כן |  |

## תגובה

מחזיר: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_badge_progress_by_id200_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_badge_progress_by_user_id'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # קביעת הרשאת מפתח ה-API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר הערה מהשורה הבאה כדי לקבוע קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 

begin
  
  result = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "שגיאה בעת קריאה ל-DefaultApi->get_user_badge_progress_by_user_id: #{e}"
end
[inline-code-end]