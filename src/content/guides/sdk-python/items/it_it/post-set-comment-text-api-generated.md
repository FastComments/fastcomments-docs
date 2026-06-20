## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio post_set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedi configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.post_set_comment_text(comment_id, set_comment_text_params, sso=sso)
        print("The response of ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]