## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_sso_user_api_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'add_sso_user उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप करें
FastCommentsClient.configure do |config|
  # API की प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_apisso_user_data = FastCommentsClient::CreateAPISSOUserData.new({email: 'email_example', username: 'username_example', id: 'id_example'}) # CreateAPISSOUserData | 

begin
  
  result = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_sso_user: #{e}"
end
[inline-code-end]