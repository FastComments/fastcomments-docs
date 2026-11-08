## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/user_reacts_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_user_reacts_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  post_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_reacts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_reacts_public: #{e}"
end
[inline-code-end]