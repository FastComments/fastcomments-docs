## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| voteId | string | path | Ja |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Returns: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_delete_response.rb)

## Example

[inline-code-attrs-start title = 'delete_moderation_vote Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
vote_id = 'vote_id_example' # String | 
opts = {
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.delete_moderation_vote(tenant_id, comment_id, vote_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Fout bij het aanroepen van ModerationApi->delete_moderation_vote: #{e}"
end
[inline-code-end]