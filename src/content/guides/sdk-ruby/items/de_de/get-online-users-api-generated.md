---
Derzeit online befindliche Betrachter einer Seite: Personen, deren WebSocket-Sitzung derzeit auf die Seite abonniert ist.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Zuschauer, die wir nicht einzeln auflisten).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Tiebreaker für den Cursor: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_online_users Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Seiten-URL-Kennung (serverseitig bereinigt).
opts = {
  after_name: 'after_name_example', # String | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
  after_user_id: 'after_user_id_example' # String | Tiebreaker für den Cursor: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---