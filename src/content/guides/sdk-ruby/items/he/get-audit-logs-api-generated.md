## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |
| order | string | query | לא |  |
| after | number | query | לא |  |
| before | number | query | לא |  |

## תגובה

מחזיר: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_audit_logs200_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_audit_logs'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # הגדר הרשאת מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # בטל את ההערה על השורה הבאה כדי להגדיר קידומת עבור מפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל: nil)
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