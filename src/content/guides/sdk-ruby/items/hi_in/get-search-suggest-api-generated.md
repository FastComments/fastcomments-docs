## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_suggest_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_search_suggest उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  text_search: 'text_search_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_search_suggest(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_suggest: #{e}"
end
[inline-code-end]