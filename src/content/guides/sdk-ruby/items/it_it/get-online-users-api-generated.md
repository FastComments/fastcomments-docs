Visualizzatori attualmente online di una pagina: persone la cui sessione websocket è iscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti nella stanza, inclusi i visualizzatori anonimi che non elenchiamo).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore dell'URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Meccanismo di spareggio del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi per nome non facciano perdere voci. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identificatore dell'URL della pagina (pulito lato server).
opts = {
  after_name: 'after_name_example', # String | Cursore: passare nextAfterName dalla risposta precedente.
  after_user_id: 'after_user_id_example' # String | Meccanismo di spareggio del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi per nome non facciano perdere voci.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]