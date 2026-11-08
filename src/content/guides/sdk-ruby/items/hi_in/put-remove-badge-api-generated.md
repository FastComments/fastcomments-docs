## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| badgeId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| commentId | string | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/remove_user_badge_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'put_remove_badge उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
badge_id = 'badge_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  comment_id: 'comment_id_example', # String | 
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.put_remove_badge(tenant_id, badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_remove_badge: #{e}"
end
[inline-code-end]