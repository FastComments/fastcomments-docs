## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_package_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'create_tenant_package उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
create_tenant_package_body = FastCommentsClient::CreateTenantPackageBody.new({name: 'name_example', max_monthly_page_loads: 3.56, max_monthly_api_credits: 3.56, max_monthly_comments: 3.56, max_concurrent_users: 3.56, max_tenant_users: 3.56, max_sso_users: 3.56, max_moderators: 3.56, max_domains: 3.56, has_debranding: false, for_who_text: 'for_who_text_example', feature_taglines: ['feature_taglines_example'], has_flex_pricing: false}) # CreateTenantPackageBody | 

begin
  
  result = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant_package: #{e}"
end
[inline-code-end]