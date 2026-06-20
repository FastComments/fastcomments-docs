## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_ban_status_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_comment_ban_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_ban_status_response import GetCommentBanStatusResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_comment_ban_status(comment_id, sso=sso)
        print("The response of ModerationApi->get_comment_ban_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_comment_ban_status: %s\n" % e)
[inline-code-end]