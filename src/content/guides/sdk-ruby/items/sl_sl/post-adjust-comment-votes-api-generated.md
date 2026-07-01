## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vrne: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/adjust_votes_response.rb)

## Primer

[inline-code-attrs-start title = 'post_adjust_comment_votes Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Niz | 
comment_id = 'comment_id_example' # Niz | 
adjust_comment_votes_params = FastCommentsClient::AdjustCommentVotesParams.new({adjust_vote_amount: 3.56}) # AdjustCommentVotesParams | 
opts = {
  broadcast_id: 'broadcast_id_example', # Niz | 
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.post_adjust_comment_votes(tenant_id, comment_id, adjust_comment_votes_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_adjust_comment_votes: #{e}"
end
[inline-code-end]