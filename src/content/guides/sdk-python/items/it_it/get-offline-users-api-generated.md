Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usare questo dopo aver esaurito /users/online per visualizzare una sezione "Members".
Paginazione con cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, in modo che le voci con lo stesso nome non vengano perse. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificatore dell'URL della pagina (pulito lato server).
    after_name = 'after_name_example' # str | Cursore: passare nextAfterName dalla risposta precedente. (optional)
    after_user_id = 'after_user_id_example' # str | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, in modo che le voci con lo stesso nome non vengano perse. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]