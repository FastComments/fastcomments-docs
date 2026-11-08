## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| deleteComments | string | query | नहीं |  |
| commentDeleteMode | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/api_empty_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'delete_tenant_user उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप करें
FastCommentsClient.configure do |config|
  # API की प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API की के लिए उपसर्ग सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  delete_comments: 'delete_comments_example', # String | 
  comment_delete_mode: 'comment_delete_mode_example' # String | 
}

begin
  
  result = api_instance.delete_tenant_user(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_tenant_user: #{e}"
end
[inline-code-end]