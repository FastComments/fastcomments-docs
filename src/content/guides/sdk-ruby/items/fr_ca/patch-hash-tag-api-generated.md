## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tag | string | chemin | Oui |  |
| tenantId | string | requête | Non |  |

## Réponse

Renvoie : [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_hash_tag200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de patch_hash_tag'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, p. ex. 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  update_hash_tag_body: FastCommentsClient::UpdateHashTagBody.new # UpdateHashTagBody | 
}

begin
  
  result = api_instance.patch_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_hash_tag: #{e}"
end
[inline-code-end]