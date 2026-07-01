## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## תשובה

מחזיר: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_hash_tag_response.rb)

## דוגמה

[inline-code-attrs-start title = 'add_hash_tag דוגמה'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # הגדרת הרשאת מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # בטלו את ההערה על השורה הבאה כדי להגדיר קידומת למפתח ה‑API, למשל 'Bearer' (ברירת מחדל היא nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_hash_tag_body = FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 

begin
  
  result = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "שגיאה בעת קריאה ל-DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]