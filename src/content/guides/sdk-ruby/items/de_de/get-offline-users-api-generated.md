Frühere Kommentatoren auf der Seite, die NICHT derzeit online sind. Sortiert nach displayName.
Verwende dies nachdem /users/online erschöpft wurde, um einen "Mitglieder"-Abschnitt darzustellen.
Cursor-Paginierung auf commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index ab afterName vorwärts via $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: Übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: Übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei Namensgleichheit keine Einträge verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_offline_users Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Seiten-URL-Kennung (serverseitig bereinigt).
opts = {
  after_name: 'after_name_example', # String | Cursor: Übergeben Sie nextAfterName aus der vorherigen Antwort.
  after_user_id: 'after_user_id_example' # String | Cursor-Tiebreaker: Übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei Namensgleichheit keine Einträge verloren gehen.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]