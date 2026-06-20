## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| skip | integer | query | Non |  |

## Réponse

Renvoie : [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_users_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_sso_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configuration de l'authentification
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API : api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 56 # Integer | 
}

begin
  
  result = api_instance.get_sso_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_sso_users: #{e}"
end
[inline-code-end]