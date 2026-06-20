## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| commentId | string | paramètre de requête | Non |  |
| sso | string | paramètre de requête | Non |  |

## Réponse

Retourne : [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_internal_profile_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_user_internal_profile'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  comment_id: 'comment_id_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_user_internal_profile(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_user_internal_profile: #{e}"
end
[inline-code-end]