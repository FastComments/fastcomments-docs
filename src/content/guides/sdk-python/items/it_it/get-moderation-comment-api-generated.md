## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| includeEmail | boolean | query | No |  |
| includeIP | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_comment_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_moderation_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_comment_response import ModerationAPICommentResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e predefinita a https://fastcomments.com
# Vedere configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_email = True # bool |  (opzionale)
    include_ip = True # bool |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_moderation_comment(comment_id, include_email=include_email, include_ip=include_ip, sso=sso)
        print("The response of ModerationApi->get_moderation_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment: %s\n" % e)
[inline-code-end]

---