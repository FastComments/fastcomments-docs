## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_moderator_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_moderator उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# authorization सेटअप करें
FastCommentsClient.configure do |config|
  # API key authorization कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API key के लिए prefix सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_moderator(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_moderator: #{e}"
end
[inline-code-end]