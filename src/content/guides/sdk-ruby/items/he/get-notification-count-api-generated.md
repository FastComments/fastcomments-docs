## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| urlId | string | query | לא |  |
| fromCommentId | string | query | לא |  |
| viewed | boolean | query | לא |  |
| type | string | query | לא |  |

## תגובה

מחזיר: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notification_count_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_notification_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת אימות
FastCommentsClient.configure do |config|
  # הגדר הרשאת מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל 'Bearer' (ברירת מחדל nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
opts = {
  user_id: 'user_id_example', # מחרוזת | 
  url_id: 'url_id_example', # מחרוזת | 
  from_comment_id: 'from_comment_id_example', # מחרוזת | 
  viewed: true, # בוליאני | 
  type: 'type_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notification_count: #{e}"
end
[inline-code-end]