## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| id | string | נתיב | כן |  |

## תגובה

מחזיר: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_user_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_tenant_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדר הרשאה
FastCommentsClient.configure do |config|
  # הגדר הרשאת מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת עבור מפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_tenant_user(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_user: #{e}"
end
[inline-code-end]