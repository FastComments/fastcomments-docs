Visualizzatori attualmente online di una pagina: persone la cui sessione websocket è iscritta alla pagina in questo momento.  
Restituisce anonCount + totalCount (abbonati a tutta la stanza, inclusi visualizzatori anonimi che non enumeriamo).

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore di sblocco: passa nextAfterUserId dalla risposta precedente. Necessario quando afterName è impostato affinché i pareggi di nome non eliminino le voci. |

## Response

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Esempio get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è facoltativo e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificatore URL della pagina (pulito lato server).
    after_name = 'after_name_example' # str | Cursore: passa nextAfterName dalla risposta precedente. (opzionale)
    after_user_id = 'after_user_id_example' # str | Cursore di sblocco: passa nextAfterUserId dalla risposta precedente. Necessario quando afterName è impostato affinché i pareggi di nome non eliminino le voci. (opzionale)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]