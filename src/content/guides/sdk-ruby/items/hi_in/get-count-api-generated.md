## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filter | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| demo | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_api_count_comments_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_count उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filter: 'filter_example', # String | 
  search_filters: 'search_filters_example', # String | 
  demo: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]