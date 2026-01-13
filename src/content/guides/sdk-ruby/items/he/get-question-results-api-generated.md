## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| startDate | string | query | לא |  |
| questionId | string | query | לא |  |
| questionIds | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_results200_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_question_results'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת המחדל היא nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  start_date: 'start_date_example', # String | 
  question_id: 'question_id_example', # String | 
  question_ids: 'question_ids_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_results: #{e}"
end
[inline-code-end]

---