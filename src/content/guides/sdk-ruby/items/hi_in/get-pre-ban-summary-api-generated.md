## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| includeByUserIdAndEmail | boolean | query | नहीं |  |
| includeByIP | boolean | query | नहीं |  |
| includeByEmailDomain | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_pre_ban_summary उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  include_by_user_id_and_email: true, # Boolean | 
  include_by_ip: true, # Boolean | 
  include_by_email_domain: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_pre_ban_summary(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]