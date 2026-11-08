## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| urlId | string | query | नहीं |  |
| fromCommentId | string | query | नहीं |  |
| viewed | boolean | query | नहीं |  |
| type | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notification_count_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_notification_count उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  user_id: 'user_id_example', # String | 
  url_id: 'url_id_example', # String | 
  from_comment_id: 'from_comment_id_example', # String | 
  viewed: true, # Boolean | 
  type: 'type_example' # String | 
}

begin
  
  result = api_instance.get_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notification_count: #{e}"
end
[inline-code-end]