Spettatori attualmente online di una pagina: persone la cui sessione websocket è sottoscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti alla stanza, inclusi gli spettatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore dell'URL della pagina (normalizzato lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Criterio di spareggio del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi di nome non eliminino voci. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Specificare l'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedere configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificatore dell'URL della pagina (normalizzato lato server).
    after_name = 'after_name_example' # str | Cursore: passa nextAfterName dalla risposta precedente. (opzionale)
    after_user_id = 'after_user_id_example' # str | Criterio di spareggio del cursore: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i duplicati di nome non escludano voci. (opzionale)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]