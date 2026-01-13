## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Retourne : [`GetTenant200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_tenant'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par exemple 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
id = 'id_example' # Chaîne | 

begin
  
  result = api_instance.get_tenant(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant: #{e}"
end
[inline-code-end]