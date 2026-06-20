## פרמטרים

| שם | Type | Location | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| badgeId | string | query | לא |  |
| type | number | query | לא |  |
| displayedOnComments | boolean | query | לא |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badges_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_badges'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # השמט את סימון ההערה מהשורה הבאה כדי לקבוע קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  badge_id: 'badge_id_example', # String | 
  type: 1.2, # Float | 
  displayed_on_comments: true, # Boolean | 
  limit: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_user_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badges: #{e}"
end
[inline-code-end]