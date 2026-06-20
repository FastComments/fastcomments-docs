## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_api_ids'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (opzionale)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (opzionale)
    filters = 'filters_example' # str |  (opzionale)
    search_filters = 'search_filters_example' # str |  (opzionale)
    after_id = 'after_id_example' # str |  (opzionale)
    demo = True # bool |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]