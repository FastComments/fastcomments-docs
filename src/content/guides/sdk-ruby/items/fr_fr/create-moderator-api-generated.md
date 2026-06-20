## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | requête | Oui |  |

## Réponse

Renvoie : [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_moderator_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_moderator'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_moderator_body = FastCommentsClient::CreateModeratorBody.new({name: 'name_example', email: 'email_example'}) # CreateModeratorBody | 

begin
  
  result = api_instance.create_moderator(tenant_id, create_moderator_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_moderator: #{e}"
end
[inline-code-end]