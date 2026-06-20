Informazioni utente in blocco per un tenant. Dati i userIds, restituisce le informazioni di visualizzazione da User / SSOUser.
Usato dal widget dei commenti per arricchire gli utenti che sono appena apparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy è applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds separati da virgola. |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_users_info'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | userIds separati da virgola.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]