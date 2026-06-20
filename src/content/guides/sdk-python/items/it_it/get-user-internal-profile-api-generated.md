## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Response

Restituisce: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_internal_profile'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e predefinito a https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_user_internal_profile(comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]

---