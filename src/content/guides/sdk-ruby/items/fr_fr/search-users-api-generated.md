## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| usernameStartsWith | string | query | Oui |  |
| mentionGroupIds | array | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
url_id = 'url_id_example' # Chaîne | 
username_starts_with = 'username_starts_with_example' # Chaîne | 
opts = {
  mention_group_ids: ['inner_example'], # Array<Chaîne> | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, username_starts_with, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]