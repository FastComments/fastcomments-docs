Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Page URL identifier (cleaned server-side). |
| afterName | string | query | Non | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Tie-breaker du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les entrées avec le même nom ne soient pas perdues. |

## Réponse

Retourne : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifiant de l'URL de la page (nettoyé côté serveur).
opts = {
  after_name: 'after_name_example', # String | Curseur : passez nextAfterName depuis la réponse précédente.
  after_user_id: 'after_user_id_example' # String | Tie-breaker du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les entrées avec le même nom ne soient pas perdues.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]