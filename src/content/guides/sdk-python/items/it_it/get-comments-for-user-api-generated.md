## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Risposta

Restituisce: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definire l'host è facoltativo e predefinito a https://fastcomments.com
# Vedere configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Aprire un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (opzionale)
    direction = client.SortDirections() # SortDirections |  (opzionale)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (opzionale)
    page = 3.4 # float |  (opzionale)
    includei10n = True # bool |  (opzionale)
    locale = 'locale_example' # str |  (opzionale)
    is_crawler = True # bool |  (opzionale)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("La risposta di PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Eccezione durante la chiamata a PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]