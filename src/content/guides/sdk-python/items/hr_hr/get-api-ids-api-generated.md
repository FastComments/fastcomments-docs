## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| afterId | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Primjer

[inline-code-attrs-start title = 'get_api_ids Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (neobavezno)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (neobavezno)
    filters = 'filters_example' # str |  (neobavezno)
    search_filters = 'search_filters_example' # str |  (neobavezno)
    after_id = 'after_id_example' # str |  (neobavezno)
    demo = True # bool |  (neobavezno)
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]