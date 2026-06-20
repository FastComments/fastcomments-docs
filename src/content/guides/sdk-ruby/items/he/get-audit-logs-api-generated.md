## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |
| order | string | query | לא |  |
| after | number | query | לא |  |
| before | number | query | לא |  |

## תגובה

מחזיר: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_audit_logs_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_audit_logs'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאות
FastCommentsClient.configure do |config|
  # הגדר אישור באמצעות מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  limit: 1.2, # Float | 
  skip: 1.2, # Float | 
  order: FastCommentsClient::SORTDIR::ASC, # SORTDIR | 
  after: 1.2, # Float | 
  before: 1.2 # Float | 
}

begin
  
  result = api_instance.get_audit_logs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_audit_logs: #{e}"
end
[inline-code-end]

---