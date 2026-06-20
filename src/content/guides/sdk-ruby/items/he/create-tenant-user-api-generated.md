## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## תגובה

מחזיר: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_user_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_tenant_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאות
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל 'Bearer' (ברירת מחדל היא nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_user_body = FastCommentsClient::CreateTenantUserBody.new({username: 'username_example', email: 'email_example'}) # CreateTenantUserBody | 

begin
  
  result = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant_user: #{e}"
end
[inline-code-end]