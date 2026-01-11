## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| updateComments | boolean | query | לא |  |

## תגובה

מחזיר: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/put_s_s_o_user_a_p_i_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-put_sso_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאות
FastCommentsClient.configure do |config|
  # קביעת אישור למפתח ה-API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להוסיף קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_apisso_user_data = FastCommentsClient::UpdateAPISSOUserData.new # UpdateAPISSOUserData | 
opts = {
  update_comments: true # Boolean | 
}

begin
  
  result = api_instance.put_sso_user(tenant_id, id, update_apisso_user_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->put_sso_user: #{e}"
end
[inline-code-end]

---