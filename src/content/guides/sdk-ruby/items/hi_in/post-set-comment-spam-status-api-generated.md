## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| spam | boolean | query | नहीं |  |
| permNotSpam | boolean | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/api_empty_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'post_set_comment_spam_status उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  spam: true, # Boolean | 
  perm_not_spam: true, # Boolean | 
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_set_comment_spam_status(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_spam_status: #{e}"
end
[inline-code-end]