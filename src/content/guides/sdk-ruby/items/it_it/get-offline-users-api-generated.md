Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usa questo dopo aver esaurito /users/online per visualizzare una sezione "Members".
Paginazione a cursore su commenterName: il server scorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore della URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identificatore della URL della pagina (ripulito lato server).
opts = {
  after_name: 'after_name_example', # String | Cursore: passare nextAfterName dalla risposta precedente.
  after_user_id: 'after_user_id_example' # String | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]