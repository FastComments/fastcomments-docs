## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תשובה

מחזיר: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמת create_tenant'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת אימות
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר הערת שורה זו כדי להגדיר קידומת למפתח ה-API, למשל 'Bearer' (ברירת המחדל היא nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_body = FastCommentsClient::CreateTenantBody.new({name: 'name_example', domain_configuration: [FastCommentsClient::APIDomainConfiguration.new({id: 'id_example', domain: 'domain_example', created_at: Time.now})]}) # CreateTenantBody | 

begin
  
  result = api_instance.create_tenant(tenant_id, create_tenant_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant: #{e}"
end
[inline-code-end]

---