## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| dir | integer | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names_success_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comment_vote_user_names'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
comment_id = 'comment_id_example' # Chaîne | 
dir = 56 # Entier | 
opts = {
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]

---