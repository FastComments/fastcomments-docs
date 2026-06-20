## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/adjust_votes_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_adjust_comment_votes'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
adjust_comment_votes_params = FastCommentsClient::AdjustCommentVotesParams.new({adjust_vote_amount: 3.56}) # AdjustCommentVotesParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_adjust_comment_votes(comment_id, adjust_comment_votes_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_adjust_comment_votes: #{e}"
end
[inline-code-end]