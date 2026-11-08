## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| trustFactor | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_user_trust_factor_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'set_trust_factor उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  trust_factor: 'trust_factor_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.set_trust_factor(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->set_trust_factor: #{e}"
end
[inline-code-end]