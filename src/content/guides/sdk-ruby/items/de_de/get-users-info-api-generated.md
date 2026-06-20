Massenhafte Benutzerinformationen für einen Mandanten. Gegebenen userIds werden Anzeigeinformationen von User / SSOUser zurückgegeben.
Vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade durch ein Präsenzereignis erschienen sind.
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Kommagetrennte userIds. |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_users_info Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Kommagetrennte userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]