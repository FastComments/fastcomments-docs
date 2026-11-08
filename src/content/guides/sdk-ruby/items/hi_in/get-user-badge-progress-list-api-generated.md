## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| limit | number | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/api_get_user_badge_progress_list_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_user_badge_progress_list उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप
FastCommentsClient.configure do |config|
  # API की प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  limit: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_user_badge_progress_list(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badge_progress_list: #{e}"
end
[inline-code-end]