## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| updateComments | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_sso_user_api_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'patch_sso_user उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
id = 'id_example' # String | 
update_apisso_user_data = FastCommentsClient::UpdateAPISSOUserData.new # UpdateAPISSOUserData | 
opts = {
  update_comments: true # Boolean | 
}

begin
  
  result = api_instance.patch_sso_user(tenant_id, id, update_apisso_user_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_sso_user: #{e}"
end
[inline-code-end]