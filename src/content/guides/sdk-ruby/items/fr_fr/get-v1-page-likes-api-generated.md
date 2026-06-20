## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui |  |

## Réponse

Renvoie : [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v1_page_likes.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_v1_page_likes'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
url_id = 'url_id_example' # Chaîne | 

begin
  
  result = api_instance.get_v1_page_likes(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v1_page_likes: #{e}"
end
[inline-code-end]

---