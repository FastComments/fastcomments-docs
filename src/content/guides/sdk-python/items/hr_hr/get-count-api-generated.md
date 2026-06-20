## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filter | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## Primjer

[inline-code-attrs-start title = 'get_count Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (neobavezno)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (neobavezno)
    filter = 'filter_example' # str |  (neobavezno)
    search_filters = 'search_filters_example' # str |  (neobavezno)
    demo = True # bool |  (neobavezno)
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.get_count(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso)
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]