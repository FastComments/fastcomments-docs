---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_banned_users_count_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_counts'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_counts(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_counts: #{e}"
end
[inline-code-end]

---