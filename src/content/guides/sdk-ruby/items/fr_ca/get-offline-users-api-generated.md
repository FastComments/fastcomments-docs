Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Members".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} depuis afterName vers l'avant via $gt, sans coût $skip.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Départageur de curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms n'entraînent pas la perte d'entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifiant de l'URL de la page (nettoyé côté serveur).
opts = {
  after_name: 'after_name_example', # String | Curseur : passez nextAfterName depuis la réponse précédente.
  after_user_id: 'after_user_id_example' # String | Départageur de curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms n'entraînent pas la perte d'entrées.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]