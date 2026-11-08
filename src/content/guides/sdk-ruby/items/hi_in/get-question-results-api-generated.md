## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionIds | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_results_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_question_results उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप करें
FastCommentsClient.configure do |config|
  # API कुंजी प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API कुंजी के लिए उपसर्ग सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
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