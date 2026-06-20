## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Chaîne | 
opts = {
  direction: 'direction_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.post_vote(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_vote: #{e}"
end
[inline-code-end]