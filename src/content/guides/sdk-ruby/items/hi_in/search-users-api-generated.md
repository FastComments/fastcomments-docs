## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| usernameStartsWith | string | query | नहीं |  |
| mentionGroupIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| searchSection | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: `SearchUsersResult`

## उदाहरण

[inline-code-attrs-start title = 'search_users उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  username_starts_with: 'username_starts_with_example', # String | 
  mention_group_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  search_section: 'fast' # String | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]