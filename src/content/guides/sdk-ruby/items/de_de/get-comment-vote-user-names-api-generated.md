## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|-------------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| dir | integer | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names200_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_comment_vote_user_names Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
dir = 56 # Integer | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]

---