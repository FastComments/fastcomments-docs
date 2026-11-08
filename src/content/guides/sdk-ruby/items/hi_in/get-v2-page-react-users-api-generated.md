## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| id | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_react_users_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_v2_page_react_users उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_react_users: #{e}"
end
[inline-code-end]