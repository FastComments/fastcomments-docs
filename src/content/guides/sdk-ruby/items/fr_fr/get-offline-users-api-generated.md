Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Membres".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} à partir de afterName en avant via $gt, sans coût $skip.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettre nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Tiebreaker de curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. |

## Réponse

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifiant de l'URL de la page (nettoyé côté serveur).
opts = {
  after_name: 'after_name_example', # String | Curseur : transmettre nextAfterName depuis la réponse précédente.
  after_user_id: 'after_user_id_example' # String | Tiebreaker de curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]