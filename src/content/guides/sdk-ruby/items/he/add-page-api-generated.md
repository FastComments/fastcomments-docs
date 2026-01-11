## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_page_a_p_i_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-add_page'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # הגדרת הרשאת מפתח API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # בטל את ההערה על השורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת המחדל nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_api_page_data = FastCommentsClient::CreateAPIPageData.new({title: 'title_example', url: 'url_example', url_id: 'url_id_example'}) # CreateAPIPageData | 

begin
  
  result = api_instance.add_page(tenant_id, create_api_page_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_page: #{e}"
end
[inline-code-end]

---