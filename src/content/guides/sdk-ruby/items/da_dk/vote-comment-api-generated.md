## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| broadcastId | string | query | Ja |  |
| sessionId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_comment200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'vote_comment Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
url_id = 'url_id_example' # String | 
broadcast_id = 'broadcast_id_example' # String | 
vote_body_params = FastCommentsClient::VoteBodyParams.new({commenter_email: 'commenter_email_example', commenter_name: 'commenter_name_example', vote_dir: 'up', url: 'url_example'}) # VoteBodyParams | 
opts = {
  session_id: 'session_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->vote_comment: #{e}"
end
[inline-code-end]

---