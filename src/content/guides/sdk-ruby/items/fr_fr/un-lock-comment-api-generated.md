## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| commentId | string | chemin | Oui |  |
| broadcastId | string | requête | Oui |  |
| sso | string | requête | Non |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de un_lock_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
comment_id = 'comment_id_example' # Chaîne | 
broadcast_id = 'broadcast_id_example' # Chaîne | 
opts = {
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.un_lock_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_lock_comment: #{e}"
end
[inline-code-end]

---