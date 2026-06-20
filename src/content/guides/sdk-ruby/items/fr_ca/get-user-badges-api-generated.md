## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| badgeId | string | query | Non |  |
| type | number | query | Non |  |
| displayedOnComments | boolean | query | Non |  |
| limit | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Retourne : [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badges_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_badges'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurer l'autorisation
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, p.ex. 'Bearer' (par défaut à nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
opts = {
  user_id: 'user_id_example', # Chaîne | 
  badge_id: 'badge_id_example', # Chaîne | 
  type: 1.2, # Flottant | 
  displayed_on_comments: true, # Booléen | 
  limit: 1.2, # Flottant | 
  skip: 1.2 # Flottant | 
}

begin
  
  result = api_instance.get_user_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badges: #{e}"
end
[inline-code-end]