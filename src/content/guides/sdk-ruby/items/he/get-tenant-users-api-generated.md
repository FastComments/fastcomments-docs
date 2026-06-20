## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_users_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_tenant_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # הגדר הרשאת מפתח ה-API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # בטל את ההערה מהשורה הבאה כדי להגדיר קידומת עבור מפתח ה-API, לדוגמה 'Bearer' (ברירת המחדל: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
opts = {
  skip: 1.2 # מספר עשרוני | 
}

begin
  
  result = api_instance.get_tenant_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_users: #{e}"
end
[inline-code-end]