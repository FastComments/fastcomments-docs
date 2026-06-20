---
Spectateurs actuellement en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés de la salle dans son ensemble, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passer nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Critère de départage du curseur : passer nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la perte d'entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifiant de l'URL de la page (nettoyé côté serveur).
opts = {
  after_name: 'after_name_example', # String | Curseur : transmettre nextAfterName depuis la réponse précédente.
  after_user_id: 'after_user_id_example' # String | Critère de départage du curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la perte d'entrées.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---