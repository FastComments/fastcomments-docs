## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| reviewed | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di post_set_comment_review_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    reviewed = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_review_status(comment_id, reviewed=reviewed, sso=sso)
        print("The response of ModerationApi->post_set_comment_review_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_review_status: %s\n" % e)
[inline-code-end]