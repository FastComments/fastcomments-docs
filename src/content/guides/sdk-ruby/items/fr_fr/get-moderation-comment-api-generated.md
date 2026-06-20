## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| includeEmail | boolean | query | Non |  |
| includeIP | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_moderation_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Chaîne | 
opts = {
  include_email: true, # Booléen | 
  include_ip: true, # Booléen | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_moderation_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]

---