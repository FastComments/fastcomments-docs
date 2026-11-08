## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/api_empty_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'update_email_template उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ऑथराइज़ेशन सेटअप करें
FastCommentsClient.configure do |config|
  # API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_email_template_body = FastCommentsClient::UpdateEmailTemplateBody.new # UpdateEmailTemplateBody | 

begin
  
  result = api_instance.update_email_template(tenant_id, id, update_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_email_template: #{e}"
end
[inline-code-end]