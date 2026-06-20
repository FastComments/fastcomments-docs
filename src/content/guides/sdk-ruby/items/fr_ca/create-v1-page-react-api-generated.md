## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui |  |
| title | string | requête | Non |  |

## Réponse

Renvoie : [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_v1_page_react.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple create_v1_page_react'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
url_id = 'url_id_example' # Chaîne | 
opts = {
  title: 'title_example' # Chaîne | 
}

begin
  
  result = api_instance.create_v1_page_react(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_v1_page_react: #{e}"
end
[inline-code-end]