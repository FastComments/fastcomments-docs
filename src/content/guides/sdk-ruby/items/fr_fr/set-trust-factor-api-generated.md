## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| trustFactor | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_user_trust_factor_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de set_trust_factor'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # Chaîne | 
  trust_factor: 'trust_factor_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.set_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->set_trust_factor: #{e}"
end
[inline-code-end]

---