## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| broadcastId | string | query | हाँ |  |
| sessionId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/save_comments_response_with_presence.rb)

## उदाहरण

[inline-code-attrs-start title = 'create_comment_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
broadcast_id = 'broadcast_id_example' # String | 
comment_data = FastCommentsClient::CommentData.new({commenter_name: 'commenter_name_example', comment: 'comment_example', url: 'url_example', url_id: 'url_id_example'}) # CommentData | 
opts = {
  session_id: 'session_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_comment_public: #{e}"
end
[inline-code-end]