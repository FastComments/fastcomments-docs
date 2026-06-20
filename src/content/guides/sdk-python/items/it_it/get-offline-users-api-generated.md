Commentatori passati sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usalo dopo aver esaurito /users/online per visualizzare una sezione "Membri".
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore per spareggio: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi dei nomi non facciano perdere voci. |

## Response

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per default è https://fastcomments.com
# Vedi configuration.py per la lista di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificatore dell'URL della pagina (pulito lato server).
    after_name = 'after_name_example' # str | Cursore: passa nextAfterName dalla risposta precedente. (opzionale)
    after_user_id = 'after_user_id_example' # str | Cursore per spareggio: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi dei nomi non facciano perdere voci. (opzionale)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]