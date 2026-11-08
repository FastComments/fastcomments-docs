## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postId | string | path | हाँ |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_feed_post_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'update_feed_post_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
post_id = 'post_id_example' # String | 
update_feed_post_params = FastCommentsClient::UpdateFeedPostParams.new # UpdateFeedPostParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_feed_post_public: #{e}"
end
[inline-code-end]