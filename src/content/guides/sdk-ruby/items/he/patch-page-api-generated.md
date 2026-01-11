## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_page_a_p_i_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-patch_page'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת אימות
FastCommentsClient.configure do |config|
  # הגדרת אישור באמצעות מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל 'Bearer' (ברירת מחדל: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_page_data = FastCommentsClient::UpdateAPIPageData.new # UpdateAPIPageData | 

begin
  
  result = api_instance.patch_page(tenant_id, id, update_api_page_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_page: #{e}"
end
[inline-code-end]

---