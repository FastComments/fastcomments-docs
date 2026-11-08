req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| afterId | string | query | नहीं |  |
| limit | integer | query | नहीं |  |
| tags | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeUserInfo | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_feed_posts_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  after_id: 'after_id_example', # String | 
  limit: 56, # Integer | 
  tags: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  is_crawler: true, # Boolean | 
  include_user_info: true # Boolean | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]